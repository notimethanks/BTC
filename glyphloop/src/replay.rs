use std::fmt::Write as _;
use std::path::Path;

use crate::input::{InputEvent, Key};
use crate::Result;

/// A recorded run: seed, terminal caps and every input event with its tick.
/// That is all that is needed to re-simulate the exact same screens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    pub seed: u64,
    pub size: (u16, u16),
    pub key_release: bool,
    /// Number of ticks simulated.
    pub ticks: u64,
    pub events: Vec<(u64, InputEvent)>,
    /// Hash of the final screen, as recorded. Checked when verifying.
    pub screen_hash: Option<u64>,
}

const MAGIC: &str = "glyphloop-replay 1";

impl Replay {
    pub fn new(seed: u64, size: (u16, u16), key_release: bool) -> Self {
        Replay { seed, size, key_release, ticks: 0, events: Vec::new(), screen_hash: None }
    }

    pub fn to_text(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "{MAGIC}");
        let _ = writeln!(s, "seed {}", self.seed);
        let _ = writeln!(s, "size {} {}", self.size.0, self.size.1);
        let _ = writeln!(s, "key_release {}", self.key_release as u8);
        let _ = writeln!(s, "ticks {}", self.ticks);
        if let Some(h) = self.screen_hash {
            let _ = writeln!(s, "screen_hash {h:016x}");
        }
        for (tick, e) in &self.events {
            let (d, k) = match e {
                InputEvent::Down(k) => ("D", k),
                InputEvent::Up(k) => ("U", k),
            };
            let _ = writeln!(s, "{tick} {d} {}", key_name(*k));
        }
        s
    }

    pub fn parse(text: &str) -> Result<Self> {
        let mut lines = text.lines();
        if lines.next() != Some(MAGIC) {
            return Err("not a glyphloop replay".into());
        }
        let mut r = Replay::new(0, (0, 0), false);
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                [] => {}
                ["seed", v] => r.seed = v.parse()?,
                ["size", w, h] => r.size = (w.parse()?, h.parse()?),
                ["key_release", v] => r.key_release = *v == "1",
                ["ticks", v] => r.ticks = v.parse()?,
                ["screen_hash", v] => r.screen_hash = Some(u64::from_str_radix(v, 16)?),
                [tick, kind, key] => {
                    let key = parse_key(key).ok_or_else(|| format!("bad key: {key}"))?;
                    let e = match *kind {
                        "D" => InputEvent::Down(key),
                        "U" => InputEvent::Up(key),
                        _ => return Err(format!("bad event: {line}").into()),
                    };
                    r.events.push((tick.parse()?, e));
                }
                _ => return Err(format!("bad line: {line}").into()),
            }
        }
        Ok(r)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        std::fs::write(path, self.to_text())?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        Self::parse(&std::fs::read_to_string(path)?)
    }
}

fn key_name(k: Key) -> String {
    match k {
        Key::Char(c) => format!("c:{}", c as u32),
        other => format!("{other:?}"),
    }
}

fn parse_key(s: &str) -> Option<Key> {
    if let Some(n) = s.strip_prefix("c:") {
        return char::from_u32(n.parse().ok()?).map(Key::Char);
    }
    Some(match s {
        "Up" => Key::Up,
        "Down" => Key::Down,
        "Left" => Key::Left,
        "Right" => Key::Right,
        "Enter" => Key::Enter,
        "Esc" => Key::Esc,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "Backspace" => Key::Backspace,
        "Shift" => Key::Shift,
        "Ctrl" => Key::Ctrl,
        "Alt" => Key::Alt,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_round_trip() {
        let mut r = Replay::new(99, (60, 22), true);
        r.ticks = 10;
        r.screen_hash = Some(0xdead_beef);
        r.events.push((0, InputEvent::Down(Key::Right)));
        r.events.push((3, InputEvent::Down(Key::Char('q'))));
        r.events.push((5, InputEvent::Up(Key::Right)));
        assert_eq!(Replay::parse(&r.to_text()).unwrap(), r);
    }
}
