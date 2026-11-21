use std::ops::BitOr;

use crate::frame::Frame;
use crate::input::Input;
use crate::rng::Rng;

/// Optional terminal features a game asks for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Features(u8);

impl Features {
    pub const NONE: Features = Features(0);
    /// Real key-release events through the Kitty keyboard protocol.
    pub const KITTY_KEYS: Features = Features(1);

    pub fn contains(self, other: Features) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for Features {
    type Output = Features;
    fn bitor(self, o: Features) -> Features {
        Features(self.0 | o.0)
    }
}

/// What to do when the terminal lacks a requested feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallback {
    /// Keep running with an approximation (e.g. simulated key release).
    Graceful,
    /// Refuse to start.
    Require,
}

#[derive(Clone, Debug)]
pub struct Config {
    /// Fixed simulation rate in ticks per second.
    pub tick_rate: u32,
    /// Size of the game canvas in cells. It is centred in the terminal.
    pub size: (u16, u16),
    pub features: Features,
    pub fallback: Fallback,
}

impl Default for Config {
    fn default() -> Self {
        Config { tick_rate: 60, size: (80, 24), features: Features::KITTY_KEYS, fallback: Fallback::Graceful }
    }
}

/// What the running terminal actually supports. Recorded in replays.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub key_release: bool,
}

/// Per-game context handed to `init` and `update`.
pub struct Ctx {
    pub input: Input,
    pub rng: Rng,
    pub(crate) tick: u64,
    pub(crate) size: (u16, u16),
    pub(crate) caps: Caps,
    pub(crate) quit: bool,
}

impl Ctx {
    pub(crate) fn new(seed: u64, size: (u16, u16), caps: Caps) -> Self {
        Ctx { input: Input::default(), rng: Rng::new(seed), tick: 0, size, caps, quit: false }
    }

    /// Ticks since start. There is deliberately no wall clock.
    pub fn tick(&self) -> u64 {
        self.tick
    }
    pub fn size(&self) -> (u16, u16) {
        self.size
    }
    pub fn caps(&self) -> Caps {
        self.caps
    }
    /// Stop the game after this tick.
    pub fn quit(&mut self) {
        self.quit = true;
    }
}

pub trait Game: Sized {
    fn config() -> Config {
        Config::default()
    }
    fn init(ctx: &mut Ctx) -> Self;
    /// Called once per fixed tick.
    fn update(&mut self, ctx: &mut Ctx);
    /// Draw the current state. The frame starts empty every time.
    fn draw(&self, frame: &mut Frame);
}
