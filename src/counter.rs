use std::sync::atomic::Ordering;
use super::{COUNTER, DISCRETE_VALUES};


pub fn fetch_and_increment() -> u32 {
    COUNTER.fetch_update(
        |c| { if c < (DISCRETE_VALUES - 1) as usize { Some(c + 1) } else { Some(0) } },
        Ordering::SeqCst,
        Ordering::SeqCst,
    ).unwrap() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_increasing_basic() {
        COUNTER.store(0 as usize, Ordering::SeqCst);
        assert_eq!(0, fetch_and_increment());
        assert_eq!(1, fetch_and_increment());
        assert_eq!(2, fetch_and_increment());
    }

    #[test]
    fn counter_increasing_rollover() {
        COUNTER.store((DISCRETE_VALUES - 1) as usize, Ordering::SeqCst);
        assert_eq!(DISCRETE_VALUES - 1, fetch_and_increment());
        assert_eq!(0, fetch_and_increment());
    }

    // TODO: Multi-thread counter tests
}

#[cfg(test)]
mod benchmarks {
    use test::Bencher;
    use super::*;

    #[bench]
    fn basic_increment(b: &mut Bencher) {
        b.iter(|| fetch_and_increment())
    }

}
