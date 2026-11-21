use crate::frame::Frame;
use crate::game::{Caps, Ctx, Game};
use crate::grid::Grid;
use crate::input::InputEvent;

/// The deterministic core shared by every backend:
/// same seed + same input events per tick = same screens.
pub struct Sim<G: Game> {
    game: G,
    ctx: Ctx,
    frame: Frame,
}

impl<G: Game> Sim<G> {
    pub fn new(seed: u64, size: (u16, u16), caps: Caps) -> Self {
        let mut ctx = Ctx::new(seed, size, caps);
        let game = G::init(&mut ctx);
        Sim { game, ctx, frame: Frame::new(size) }
    }

    /// Advance one tick with the input events that arrived during it.
    pub fn step(&mut self, events: &[InputEvent]) {
        self.ctx.input.begin_tick(events);
        self.game.update(&mut self.ctx);
        self.ctx.tick += 1;
    }

    pub fn render(&mut self) -> Grid {
        self.frame.clear_all();
        self.game.draw(&mut self.frame);
        self.frame.composite()
    }

    pub fn tick(&self) -> u64 {
        self.ctx.tick
    }
    pub fn is_quitting(&self) -> bool {
        self.ctx.quit
    }
    pub fn game(&self) -> &G {
        &self.game
    }
}
