// rng.rs
// A tiny xorshift random number generator, so the project needs no crates.

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    state: u64,
}

impl Rng {
    // Seeds from the clock. The salt keeps generators made at the same
    // moment (for example one per thread) from giving the same numbers.
    pub fn new(salt: u64) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x1234_5678);
        let state = nanos ^ salt.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        Rng {
            state: if state == 0 { 0xDEAD_BEEF } else { state },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    // Returns a number from lo to hi, including both ends.
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }
}
