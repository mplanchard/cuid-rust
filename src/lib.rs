#![feature(test)]  // used for benchmarking
#![feature(no_more_cas)]  // used by counter
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize};

extern crate hostname;
extern crate rand;
extern crate test;

mod counter;
mod fingerprint;
mod random;
mod text;
mod time;

static COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
static BASE: u8 = 36;
static BLOCK_SIZE: u8 = 4;
static DISCRETE_VALUES: u32 = 1679616;  // BASE^BLOCK_SIZE
static START_STR: &str = "c";


pub fn cuid() -> String {
    [
        START_STR,
        &time::timestamp(),
        &text::to_base_str(counter::fetch_and_increment()),
        &fingerprint::fingerprint(),
        &random::random_block(),
        &random::random_block(),
    ].concat()
}


pub fn slug() -> String {
    let timestamp = time::timestamp();
    let counter = text::to_base_str(counter::fetch_and_increment());
    let fp = fingerprint::fingerprint();
    let rand = random::random_block();
    [
        &timestamp[timestamp.len()-2..],
        &counter[counter.len().saturating_sub(4)..],
        &fp[..1],
        &fp[fp.len()-1..],
        &rand[rand.len()-2..],
    ].concat()
}


pub fn is_cuid<S: Into<String>>(to_check: S) -> bool {
    &to_check.into()[..1] == START_STR
}


pub fn is_slug<S: Into<String>>(to_check: S) -> bool {
    let length = to_check.into().len();
    length >= 7 && length <=10
}


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

    #[test]
    fn cuid_is_cuid() {
        assert!(is_cuid(cuid()));
    }

    #[test]
    fn slug_max_len() {
        assert!(slug().len() <= 10);
    }

    #[test]
    fn slug_min_len() {
        assert!(slug().len() >= 7);
    }

    #[test]
    fn slug_is_slug() {
        assert!(is_slug(slug()));
    }

}
