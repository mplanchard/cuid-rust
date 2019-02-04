use std::sync::atomic::Ordering;
use error::CuidError;
use text::to_base_str;
use super::{COUNTER, DISCRETE_VALUES};


fn fetch_and_increment() -> Result<u32, CuidError> {
    COUNTER.fetch_update(
        |c| { if c < (DISCRETE_VALUES - 1) as usize { Some(c + 1) } else { Some(0) } },
        Ordering::SeqCst,
        Ordering::SeqCst,
    ).map(|res| res as u32)
    .map_err(|_| CuidError::CounterError)
}


pub fn current() -> Result<Box<str>, CuidError> {
    fetch_and_increment().map(to_base_str)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_increasing_basic() {
        COUNTER.store(0 as usize, Ordering::SeqCst);
        assert_eq!(0, fetch_and_increment().unwrap());
        assert_eq!(1, fetch_and_increment().unwrap());
        assert_eq!(2, fetch_and_increment().unwrap());
    }

    #[test]
    fn counter_increasing_rollover() {
        COUNTER.store((DISCRETE_VALUES - 1) as usize, Ordering::SeqCst);
        assert_eq!(DISCRETE_VALUES - 1, fetch_and_increment().unwrap());
        assert_eq!(0, fetch_and_increment().unwrap());
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
