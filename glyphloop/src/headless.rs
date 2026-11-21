//! Run a game without a terminal: for tests, replays and verification.
//!
//! Today this composites the screen in-process. The plan is to also pipe the
//! native backend's escape-sequence output through libghostty-vt, so the
//! renderer itself is checked against a real terminal emulator.

use crate::game::{Caps, Game};
use crate::grid::Grid;
use crate::input::InputEvent;
use crate::replay::Replay;
use crate::sim::Sim;

pub struct Headless<G: Game> {
    sim: Sim<G>,
}

impl<G: Game> Headless<G> {
    /// Headless run that behaves as if the terminal supports key release.
    pub fn new(seed: u64) -> Self {
        Self::with_caps(seed, Caps { key_release: true })
    }

    pub fn with_caps(seed: u64, caps: Caps) -> Self {
        Headless { sim: Sim::new(seed, G::config().size, caps) }
    }

    pub fn step(&mut self, events: &[InputEvent]) -> &mut Self {
        self.sim.step(events);
        self
    }

    pub fn run_ticks(&mut self, n: u64) -> &mut Self {
        for _ in 0..n {
            self.sim.step(&[]);
        }
        self
    }

    pub fn screen(&mut self) -> Grid {
        self.sim.render()
    }
    pub fn game(&self) -> &G {
        self.sim.game()
    }
    pub fn tick(&self) -> u64 {
        self.sim.tick()
    }
}

/// Re-simulate a replay and return the final screen.
pub fn play<G: Game>(replay: &Replay) -> Grid {
    let mut sim = Sim::<G>::new(replay.seed, replay.size, Caps { key_release: replay.key_release });
    let mut idx = 0;
    let mut events = Vec::new();
    for tick in 0..replay.ticks {
        events.clear();
        while idx < replay.events.len() && replay.events[idx].0 == tick {
            events.push(replay.events[idx].1);
            idx += 1;
        }
        sim.step(&events);
        if sim.is_quitting() {
            break;
        }
    }
    sim.render()
}
