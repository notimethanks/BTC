//! cargo run --example gem_garden -- [--seed N] [--record run.glr]
//! cargo run --example gem_garden -- --replay run.glr

mod game;

use game::GemGarden;
use glyphloop::prelude::*;

const USAGE: &str = "usage: gem_garden [--seed N] [--record FILE] | --replay FILE";

fn main() -> glyphloop::Result<()> {
    let mut opts = RunOptions::default();
    let mut replay_path = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--seed" => opts.seed = Some(args.next().ok_or(USAGE)?.parse()?),
            "--record" => opts.record = Some(args.next().ok_or(USAGE)?.into()),
            "--replay" => replay_path = Some(args.next().ok_or(USAGE)?),
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            other => return Err(format!("unknown argument {other}\n{USAGE}").into()),
        }
    }

    if let Some(path) = replay_path {
        // Headless: re-simulate and check the final screen.
        let replay = Replay::load(&path)?;
        let screen = glyphloop::headless::play::<GemGarden>(&replay);
        println!("{screen}\n");
        let hash = screen.hash();
        println!("seed {}  ticks {}  screen {hash:016x}", replay.seed, replay.ticks);
        match replay.screen_hash {
            Some(h) if h == hash => println!("verified: matches the recorded final screen"),
            Some(h) => {
                println!("MISMATCH: replay recorded {h:016x}");
                std::process::exit(1);
            }
            None => println!("(replay has no recorded screen hash)"),
        }
        return Ok(());
    }

    glyphloop::run_with::<GemGarden>(opts)
}
