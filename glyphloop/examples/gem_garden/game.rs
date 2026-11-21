//! Gem Garden: collect every gem, dodge the wandering slimes.

use glyphloop::prelude::*;

pub const MAP_W: i32 = 60;
pub const MAP_H: i32 = 20;
pub const START: IVec2 = ivec2(3, 3);

const GEMS: usize = 12;
const SLIMES: usize = 4;
const MOVE_COOLDOWN: u32 = 6; // ticks between steps while a key is held
const SLIME_PERIOD: u64 = 20; // slimes step every N ticks
const INVULN_TICKS: u32 = 60;
const MAX_HP: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Playing,
    Won,
    Lost,
}

pub struct GemGarden {
    pub walls: Vec<bool>,
    pub gems: Vec<IVec2>,
    pub slimes: Vec<IVec2>,
    pub player: IVec2,
    pub hp: u8,
    pub collected: usize,
    pub level: u32,
    pub state: State,
    cooldown: u32,
    invuln: u32,
    tick: u64,
    key_release: bool,
}

fn idx(p: IVec2) -> usize {
    (p.y * MAP_W + p.x) as usize
}

impl GemGarden {
    fn new_level(ctx: &mut Ctx, level: u32) -> Self {
        let mut walls = vec![false; (MAP_W * MAP_H) as usize];
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                if x == 0 || y == 0 || x == MAP_W - 1 || y == MAP_H - 1 {
                    walls[idx(ivec2(x, y))] = true;
                }
            }
        }
        // Short wall segments, more on later levels.
        for _ in 0..(16 + level as usize * 2).min(40) {
            let (cx, cy) = (ctx.rng.range(2, MAP_W - 3), ctx.rng.range(2, MAP_H - 3));
            let len = ctx.rng.range(2, 7);
            let horizontal = ctx.rng.chance(1, 2);
            for i in 0..len {
                let p = if horizontal { ivec2(cx + i, cy) } else { ivec2(cx, cy + i) };
                if p.x > 0 && p.y > 0 && p.x < MAP_W - 1 && p.y < MAP_H - 1 {
                    walls[idx(p)] = true;
                }
            }
        }
        // Keep the start area open.
        for y in 2..=4 {
            for x in 2..=9 {
                walls[idx(ivec2(x, y))] = false;
            }
        }

        let mut g = GemGarden {
            walls,
            gems: Vec::new(),
            slimes: Vec::new(),
            player: START,
            hp: MAX_HP,
            collected: 0,
            level,
            state: State::Playing,
            cooldown: 0,
            invuln: 0,
            tick: ctx.tick(),
            key_release: ctx.caps().key_release,
        };
        for _ in 0..GEMS {
            let p = g.free_cell(ctx);
            g.gems.push(p);
        }
        for _ in 0..SLIMES + level as usize {
            let p = g.free_cell(ctx);
            g.slimes.push(p);
        }
        g
    }

    pub fn wall(&self, p: IVec2) -> bool {
        p.x < 0 || p.y < 0 || p.x >= MAP_W || p.y >= MAP_H || self.walls[idx(p)]
    }

    fn free_cell(&self, ctx: &mut Ctx) -> IVec2 {
        loop {
            let p = ivec2(ctx.rng.range(1, MAP_W - 1), ctx.rng.range(1, MAP_H - 1));
            if !self.wall(p) && p.manhattan(START) > 8 && !self.gems.contains(&p) && !self.slimes.contains(&p) {
                return p;
            }
        }
    }

    fn move_slimes(&mut self, ctx: &mut Ctx) {
        const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)];
        for i in 0..self.slimes.len() {
            let s = self.slimes[i];
            let step = if ctx.rng.chance(1, 2) {
                // Drift toward the player along the longer axis.
                let d = self.player - s;
                if d.x.abs() > d.y.abs() { ivec2(d.x.signum(), 0) } else { ivec2(0, d.y.signum()) }
            } else {
                *ctx.rng.pick(&DIRS).unwrap()
            };
            let n = s + step;
            if !self.wall(n) && !self.slimes.contains(&n) {
                self.slimes[i] = n;
            }
        }
    }
}

impl Game for GemGarden {
    fn config() -> Config {
        Config {
            tick_rate: 60,
            size: (MAP_W as u16, MAP_H as u16 + 2),
            features: Features::KITTY_KEYS,
            fallback: Fallback::Graceful,
        }
    }

    fn init(ctx: &mut Ctx) -> Self {
        GemGarden::new_level(ctx, 1)
    }

    fn update(&mut self, ctx: &mut Ctx) {
        self.tick = ctx.tick();
        let i = &ctx.input;
        let quit = i.pressed(Key::Esc) || i.pressed(Key::Char('q'));
        let restart = i.pressed(Key::Char('r')) || (self.state != State::Playing && i.pressed(Key::Enter));
        let axis = |a: Key, b: Key| (i.pressed(a) || i.pressed(b), i.held(a) || i.held(b) || i.pressed(a) || i.pressed(b));
        let (lp, lh) = axis(Key::Left, Key::Char('a'));
        let (rp, rh) = axis(Key::Right, Key::Char('d'));
        let (up, uh) = axis(Key::Up, Key::Char('w'));
        let (dp, dh) = axis(Key::Down, Key::Char('s'));

        if quit {
            ctx.quit();
            return;
        }
        if restart {
            let level = if self.state == State::Won { self.level + 1 } else { self.level };
            *self = GemGarden::new_level(ctx, level);
            return;
        }
        if self.state != State::Playing {
            return;
        }

        self.cooldown = self.cooldown.saturating_sub(1);
        self.invuln = self.invuln.saturating_sub(1);

        // A fresh press always steps at once; holding repeats on a cooldown.
        let dx = rh as i32 - lh as i32;
        let dy = dh as i32 - uh as i32;
        if (dx, dy) != (0, 0) && (lp || rp || up || dp || self.cooldown == 0) {
            let nx = self.player + ivec2(dx, 0);
            if !self.wall(nx) {
                self.player = nx;
            }
            let ny = self.player + ivec2(0, dy);
            if !self.wall(ny) {
                self.player = ny;
            }
            self.cooldown = MOVE_COOLDOWN;
        }

        if let Some(at) = self.gems.iter().position(|g| *g == self.player) {
            self.gems.remove(at);
            self.collected += 1;
            if self.gems.is_empty() {
                self.state = State::Won;
            }
        }

        if self.tick > 0 && self.tick % SLIME_PERIOD == 0 {
            self.move_slimes(ctx);
        }

        if self.invuln == 0 && self.slimes.contains(&self.player) {
            self.hp -= 1;
            self.invuln = INVULN_TICKS;
            if self.hp == 0 {
                self.state = State::Lost;
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let map = f.layer(Layer::Map);
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                let cell = if self.walls[idx(ivec2(x, y))] {
                    Cell::new('#').fg(Color::Gray)
                } else {
                    Cell::new('.').fg(Color::DarkGray)
                };
                map.put((x, y), cell);
            }
        }

        let actors = f.layer(Layer::Actors);
        for g in &self.gems {
            actors.put(*g, Cell::new('*').fg(Color::Cyan).bold());
        }
        for s in &self.slimes {
            actors.put(*s, Cell::new('s').fg(Color::Green).bold());
        }
        let blink = self.invuln > 0 && (self.tick / 4) % 2 == 0;
        let pc = if blink { Color::Red } else { Color::Yellow };
        actors.put(self.player, Cell::new('@').fg(pc).bold());

        let ui = f.layer(Layer::Ui);
        let status = format!(
            " HP {}/{}   Gems {}/{}   Level {}   keys: {}",
            self.hp,
            MAX_HP,
            self.collected,
            self.collected + self.gems.len(),
            self.level,
            if self.key_release { "kitty (real key-up)" } else { "legacy (simulated)" },
        );
        ui.text((0, MAP_H), &status, Style::fg(Color::White));
        let (msg, style) = match self.state {
            State::Playing => (" arrows/WASD move  r new map  q quit".to_string(), Style::fg(Color::DarkGray)),
            State::Won => (" All gems collected! Enter: next level  q quit".to_string(), Style::fg(Color::Cyan).bold()),
            State::Lost => (" The slimes got you. Enter: retry  q quit".to_string(), Style::fg(Color::Red).bold()),
        };
        ui.text((0, MAP_H + 1), &msg, style);
    }
}
