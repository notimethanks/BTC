# Glyphloop

A small, deterministic game engine for modern terminals.

- **Fixed-tick loop** (60 Hz by default) with no wall clock in game code.
- **Real key up/down** through the Kitty keyboard protocol (Ghostty, Kitty,
  WezTerm, foot...). Older terminals fall back to a simulated key release.
- **Layered cell drawing** (`Map`, `Actors`, `Ui`), centred in the terminal.
  Only changed cells are sent.
- **Seeded RNG** as the only source of randomness.
- **Replays**: seed + input events. They re-simulate headlessly to the exact
  same final screen, checked by a stable screen hash.

## Try the demo

```sh
cargo run --example gem_garden                      # play
cargo run --example gem_garden -- --seed 42 --record run.glr
cargo run --example gem_garden -- --replay run.glr  # headless verify
cargo test
```

*Gem Garden*: collect every `*`, avoid the `s` slimes. Arrows/WASD move,
`r` new map, `q` quit.

## Writing a game

```rust
use glyphloop::prelude::*;

struct Hello { pos: IVec2 }

impl Game for Hello {
    fn init(_ctx: &mut Ctx) -> Self { Hello { pos: ivec2(5, 5) } }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.input.pressed(Key::Right) { self.pos.x += 1; }
        if ctx.input.pressed(Key::Char('q')) { ctx.quit(); }
    }

    fn draw(&self, f: &mut Frame) {
        f.layer(Layer::Actors).put(self.pos, Cell::new('@').fg(Color::Yellow));
    }
}

fn main() -> glyphloop::Result<()> { glyphloop::run::<Hello>() }
```

Test it without a terminal:

```rust
let mut h = Headless::<Hello>::new(1);
h.step(&[InputEvent::Down(Key::Right)]);
assert_eq!(h.screen().row_text(5), "      @");
```

## Layout

| File | Role |
|---|---|
| `src/game.rs` | `Game` trait, `Config`, `Ctx`, `Caps` |
| `src/sim.rs` | Deterministic core shared by all backends |
| `src/input.rs` | `pressed` / `held` / `released` per tick |
| `src/frame.rs`, `src/grid.rs`, `src/style.rs` | Layers, cells, screen hash |
| `src/replay.rs` | Replay format (plain text) |
| `src/headless.rs` | Run without a terminal |
| `src/term.rs` | Native backend (crossterm) |

## Roadmap

1. **libghostty-vt in the headless backend**: feed the native renderer's
   escape-sequence output through libghostty, to check that the bytes actually
   produce the expected screen.
2. **Image sprites** through the Kitty graphics protocol, with text fallbacks.
3. **Web backend** (WASM + ghostty-web), same game and same replays.
4. **Signed replays**: sign `seed + inputs + screen hash` with an SSH key.
