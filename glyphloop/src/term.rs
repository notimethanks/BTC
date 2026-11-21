//! Native terminal backend (crossterm).

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyboardEnhancementFlags,
    ModifierKeyCode, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::style::{Attribute, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, execute, queue};

use crate::game::{Caps, Fallback, Features, Game};
use crate::grid::Grid;
use crate::input::{InputEvent, Key};
use crate::replay::Replay;
use crate::sim::Sim;
use crate::style::{Color, Style};
use crate::Result;

#[derive(Clone, Debug, Default)]
pub struct RunOptions {
    /// Fixed seed. Random if not set.
    pub seed: Option<u64>,
    /// Save a replay to this path when the game ends.
    pub record: Option<PathBuf>,
}

/// Run a game in the current terminal.
pub fn run<G: Game>() -> Result<()> {
    run_with::<G>(RunOptions::default())
}

/// Restores the terminal on exit, including on panic.
struct TermGuard {
    enhanced: bool,
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        if self.enhanced {
            let _ = execute!(out, PopKeyboardEnhancementFlags);
        }
        let _ = execute!(out, ResetColor, cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

pub fn run_with<G: Game>(opts: RunOptions) -> Result<()> {
    let config = G::config();
    let seed = opts.seed.unwrap_or_else(time_seed);
    let mut out = io::stdout();

    terminal::enable_raw_mode()?;
    let mut guard = TermGuard { enhanced: false };
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide, terminal::Clear(ClearType::All))?;

    let wants_keys = config.features.contains(Features::KITTY_KEYS);
    let key_release = wants_keys && terminal::supports_keyboard_enhancement().unwrap_or(false);
    if wants_keys && !key_release && config.fallback == Fallback::Require {
        drop(guard);
        return Err("this game needs a terminal with the Kitty keyboard protocol \
                    (Ghostty, Kitty, WezTerm, foot, recent iTerm2...)"
            .into());
    }
    if key_release {
        execute!(
            out,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            )
        )?;
        guard.enhanced = true;
    }

    let size = config.size;
    let mut sim = Sim::<G>::new(seed, size, Caps { key_release });
    let mut replay = Replay::new(seed, size, key_release);
    let mut renderer = Renderer::new(size);

    let tick_dur = Duration::from_secs_f64(1.0 / config.tick_rate.max(1) as f64);
    // Without key-release events, a key counts as held until no repeat
    // has arrived for this many ticks.
    let hold_ticks = (config.tick_rate / 8).max(2) as u64;
    let mut fallback_holds: HashMap<Key, u64> = HashMap::new();

    let mut events = Vec::new();
    let mut force_quit = false;
    let mut next = Instant::now() + tick_dur;
    renderer.draw(&mut out, &sim.render())?;

    loop {
        let tick = sim.tick();
        events.clear();

        // Gather input until this tick's deadline.
        loop {
            let now = Instant::now();
            if now >= next || !event::poll(next - now)? {
                break;
            }
            match event::read()? {
                Event::Key(k) => {
                    if is_ctrl_c(&k) {
                        force_quit = true;
                        continue;
                    }
                    let Some(key) = map_key(k.code) else { continue };
                    if key_release {
                        match k.kind {
                            KeyEventKind::Press => events.push(InputEvent::Down(key)),
                            KeyEventKind::Release => events.push(InputEvent::Up(key)),
                            KeyEventKind::Repeat => {}
                        }
                    } else if k.kind != KeyEventKind::Release {
                        if !fallback_holds.contains_key(&key) {
                            events.push(InputEvent::Down(key));
                        }
                        fallback_holds.insert(key, tick + hold_ticks);
                    }
                }
                Event::Resize(..) => renderer.invalidate(),
                _ => {}
            }
        }
        if force_quit {
            break;
        }
        if !key_release {
            let expired: Vec<Key> =
                fallback_holds.iter().filter(|(_, &until)| until <= tick).map(|(k, _)| *k).collect();
            for k in expired {
                fallback_holds.remove(&k);
                events.push(InputEvent::Up(k));
            }
        }

        // Synthetic events are recorded too, so replays stay exact.
        replay.events.extend(events.iter().map(|e| (tick, *e)));
        sim.step(&events);
        replay.ticks = sim.tick();

        let grid = sim.render();
        if sim.is_quitting() {
            break;
        }
        renderer.draw(&mut out, &grid)?;

        next += tick_dur;
        let now = Instant::now();
        if now > next + tick_dur * 5 {
            next = now + tick_dur; // fell far behind; don't try to catch up
        }
    }

    replay.screen_hash = Some(sim.render().hash());
    drop(guard);

    if let Some(path) = opts.record {
        replay.save(&path)?;
        eprintln!("replay saved to {} ({} ticks, seed {})", path.display(), replay.ticks, replay.seed);
    }
    Ok(())
}

/// Draws a grid centred in the terminal, sending only cells that changed.
struct Renderer {
    size: (u16, u16),
    prev: Option<Grid>,
    too_small_at: Option<(u16, u16)>,
}

impl Renderer {
    fn new(size: (u16, u16)) -> Self {
        Renderer { size, prev: None, too_small_at: None }
    }

    fn invalidate(&mut self) {
        self.prev = None;
    }

    fn draw(&mut self, out: &mut impl Write, grid: &Grid) -> io::Result<()> {
        let (tw, th) = terminal::size()?;
        let (w, h) = self.size;
        if tw < w || th < h {
            if self.too_small_at != Some((tw, th)) {
                queue!(
                    out,
                    ResetColor,
                    terminal::Clear(ClearType::All),
                    cursor::MoveTo(0, 0),
                    Print(format!("Terminal too small: need {w}x{h}, have {tw}x{th}"))
                )?;
                out.flush()?;
                self.too_small_at = Some((tw, th));
            }
            self.prev = None;
            return Ok(());
        }
        self.too_small_at = None;

        let (ox, oy) = ((tw - w) / 2, (th - h) / 2);
        if self.prev.is_none() {
            queue!(out, ResetColor, terminal::Clear(ClearType::All))?;
        }
        let mut style: Option<Style> = None;
        let mut cursor_at: Option<(u16, u16)> = None;
        for y in 0..h {
            for x in 0..w {
                let c = *grid.get(x, y).expect("grid matches canvas size");
                if self.prev.as_ref().and_then(|p| p.get(x, y)) == Some(&c) {
                    continue;
                }
                if cursor_at != Some((x, y)) {
                    queue!(out, cursor::MoveTo(ox + x, oy + y))?;
                }
                if style != Some(c.style) {
                    queue!(
                        out,
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(to_ct(c.style.fg)),
                        SetBackgroundColor(to_ct(c.style.bg))
                    )?;
                    if c.style.bold {
                        queue!(out, SetAttribute(Attribute::Bold))?;
                    }
                    style = Some(c.style);
                }
                queue!(out, Print(c.ch))?;
                cursor_at = Some((x + 1, y));
            }
        }
        out.flush()?;
        self.prev = Some(grid.clone());
        Ok(())
    }
}

fn is_ctrl_c(k: &KeyEvent) -> bool {
    k.kind != KeyEventKind::Release
        && k.modifiers.contains(KeyModifiers::CONTROL)
        && matches!(k.code, KeyCode::Char('c') | KeyCode::Char('C'))
}

fn map_key(code: KeyCode) -> Option<Key> {
    Some(match code {
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Esc,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Char(' ') => Key::Space,
        KeyCode::Char(c) => Key::Char(c.to_ascii_lowercase()),
        KeyCode::Modifier(m) => match m {
            ModifierKeyCode::LeftShift | ModifierKeyCode::RightShift => Key::Shift,
            ModifierKeyCode::LeftControl | ModifierKeyCode::RightControl => Key::Ctrl,
            ModifierKeyCode::LeftAlt | ModifierKeyCode::RightAlt => Key::Alt,
            _ => return None,
        },
        _ => return None,
    })
}

fn to_ct(c: Color) -> crossterm::style::Color {
    use crossterm::style::Color as C;
    match c {
        Color::Reset => C::Reset,
        Color::Black => C::Black,
        Color::Red => C::Red,
        Color::Green => C::Green,
        Color::Yellow => C::Yellow,
        Color::Blue => C::Blue,
        Color::Magenta => C::Magenta,
        Color::Cyan => C::Cyan,
        Color::White => C::White,
        Color::Gray => C::Grey,
        Color::DarkGray => C::DarkGrey,
        Color::Rgb(r, g, b) => C::Rgb { r, g, b },
    }
}

fn time_seed() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(1)
}
