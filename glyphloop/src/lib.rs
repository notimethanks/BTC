//! Glyphloop: a small, deterministic game engine for modern terminals.
//!
//! Games implement [`Game`](game::Game). The engine runs them on a fixed tick,
//! with real key up/down input (Kitty keyboard protocol, with a fallback),
//! layered cell drawing, a seeded RNG, and replays that can be re-simulated
//! headlessly to reproduce the exact final screen.

pub mod frame;
pub mod game;
pub mod geom;
pub mod grid;
pub mod headless;
pub mod input;
pub mod replay;
pub mod rng;
pub mod sim;
pub mod style;
pub mod term;

pub use term::{run, run_with, RunOptions};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod prelude {
    pub use crate::frame::{Frame, Layer};
    pub use crate::game::{Caps, Config, Ctx, Fallback, Features, Game};
    pub use crate::geom::{ivec2, IVec2};
    pub use crate::grid::Grid;
    pub use crate::headless::Headless;
    pub use crate::input::{InputEvent, Key};
    pub use crate::replay::Replay;
    pub use crate::style::{Cell, Color, Style};
    pub use crate::{run, run_with, RunOptions};
}
