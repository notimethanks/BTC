#[path = "../examples/gem_garden/game.rs"]
mod game;

use game::{GemGarden, START};
use glyphloop::prelude::*;

#[test]
fn holding_a_key_repeats_on_cooldown() {
    let mut h = Headless::<GemGarden>::new(1);
    h.step(&[InputEvent::Down(Key::Right)]); // tick 0: immediate step
    h.run_ticks(12); // ticks 1..=12: steps at 6 and 12
    h.step(&[InputEvent::Up(Key::Right)]);
    h.run_ticks(30);
    assert_eq!(h.game().player, START + ivec2(3, 0));
}

#[test]
fn same_seed_and_inputs_give_identical_screens() {
    let run = || {
        let mut h = Headless::<GemGarden>::new(1234);
        h.step(&[InputEvent::Down(Key::Down)]).run_ticks(40);
        h.step(&[InputEvent::Up(Key::Down), InputEvent::Down(Key::Right)]).run_ticks(200);
        h.screen()
    };
    let (a, b) = (run(), run());
    assert_eq!(a.hash(), b.hash());
    assert!(a.row_text(20).contains("HP"));
}

#[test]
fn different_seeds_give_different_maps() {
    let a = Headless::<GemGarden>::new(1).screen();
    let b = Headless::<GemGarden>::new(2).screen();
    assert_ne!(a.hash(), b.hash());
}

#[test]
fn replay_round_trip_reproduces_final_screen() {
    let mut r = Replay::new(77, GemGarden::config().size, true);
    r.events = vec![
        (0, InputEvent::Down(Key::Right)),
        (25, InputEvent::Up(Key::Right)),
        (26, InputEvent::Down(Key::Down)),
        (60, InputEvent::Up(Key::Down)),
    ];
    r.ticks = 300;
    let first = glyphloop::headless::play::<GemGarden>(&r);
    let reloaded = Replay::parse(&r.to_text()).unwrap();
    assert_eq!(glyphloop::headless::play::<GemGarden>(&reloaded), first);
}
