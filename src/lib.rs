#![feature(no_more_cas)]
use std::sync::atomic::AtomicUsize;

extern crate hostname;
extern crate rand;

mod counter;
mod fingerprint;
mod random;
mod text;
mod time;

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static BASE: u8 = 36;
static BLOCK_SIZE: u8 = 4;
static DISCRETE_VALUES: u32 = 1679616;  // BASE^BLOCK_SIZE



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_discrete_values() {
        assert_eq!(
            (BASE as u32).pow(BLOCK_SIZE as u32),
            DISCRETE_VALUES,
        );
    }

}
