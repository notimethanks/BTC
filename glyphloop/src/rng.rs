/// Seeded SplitMix64 RNG. The only randomness a game should use,
/// so that replays reproduce exactly.
#[derive(Clone, Debug)]
pub struct Rng {
    seed: u64,
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng { seed, state: seed }
    }

    /// The seed this RNG started from.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform integer in `lo..hi` (exclusive). Returns `lo` if the range is empty.
    pub fn range(&mut self, lo: i32, hi: i32) -> i32 {
        if hi <= lo {
            return lo;
        }
        let span = (hi as i64 - lo as i64) as u64;
        lo + (self.next_u64() % span) as i32
    }

    /// True with probability `num / den`.
    pub fn chance(&mut self, num: u32, den: u32) -> bool {
        den > 0 && (self.next_u64() % den as u64) < num as u64
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            Some(&items[(self.next_u64() % items.len() as u64) as usize])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn range_stays_in_bounds() {
        let mut r = Rng::new(7);
        for _ in 0..1000 {
            let v = r.range(-3, 5);
            assert!((-3..5).contains(&v));
        }
    }
}
