use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Space,
    Tab,
    Backspace,
    Shift,
    Ctrl,
    Alt,
    /// A character key, always lowercase.
    Char(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputEvent {
    Down(Key),
    Up(Key),
}

/// Keyboard state for the current tick.
#[derive(Default, Debug)]
pub struct Input {
    held: HashSet<Key>,
    pressed: HashSet<Key>,
    released: HashSet<Key>,
}

impl Input {
    /// Key is currently down.
    pub fn held(&self, k: Key) -> bool {
        self.held.contains(&k)
    }
    /// Key went down this tick.
    pub fn pressed(&self, k: Key) -> bool {
        self.pressed.contains(&k)
    }
    /// Key came up this tick.
    pub fn released(&self, k: Key) -> bool {
        self.released.contains(&k)
    }

    pub(crate) fn begin_tick(&mut self, events: &[InputEvent]) {
        self.pressed.clear();
        self.released.clear();
        for e in events {
            match *e {
                InputEvent::Down(k) => {
                    if self.held.insert(k) {
                        self.pressed.insert(k);
                    }
                }
                InputEvent::Up(k) => {
                    if self.held.remove(&k) {
                        self.released.insert(k);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn press_hold_release() {
        let mut i = Input::default();
        i.begin_tick(&[InputEvent::Down(Key::Left)]);
        assert!(i.pressed(Key::Left) && i.held(Key::Left));
        i.begin_tick(&[]);
        assert!(!i.pressed(Key::Left) && i.held(Key::Left));
        i.begin_tick(&[InputEvent::Up(Key::Left)]);
        assert!(i.released(Key::Left) && !i.held(Key::Left));
    }

    #[test]
    fn tap_within_one_tick() {
        let mut i = Input::default();
        i.begin_tick(&[InputEvent::Down(Key::Space), InputEvent::Up(Key::Space)]);
        assert!(i.pressed(Key::Space) && i.released(Key::Space) && !i.held(Key::Space));
    }
}
