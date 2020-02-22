use std::convert::TryInto;
use std::sync::atomic::Ordering;

use crate::error::CuidError;
use crate::text::to_base_string;
use crate::{COUNTER, DISCRETE_VALUES};


/// Fetch the counter value and increment it.
///
/// If the counter has reached its max (DISCRETE VALUES), reset it to 0.
fn fetch_and_increment() -> Result<u32, CuidError> {
    COUNTER.compare_and_swap(
        DISCRETE_VALUES.try_into()?,
        0,
        Ordering::SeqCst,
    );
    Ok(COUNTER.fetch_add(1, Ordering::SeqCst).try_into()?)
}


/// Return the current counter value in the appropriate base as a String.
pub fn current() -> Result<String, CuidError> {
    fetch_and_increment().map(to_base_string)?
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_increasing_basic() {
        COUNTER.store(0, Ordering::SeqCst);
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

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use test::Bencher;
    use super::*;

    #[bench]
    fn basic_increment(b: &mut Bencher) {
        b.iter(|| fetch_and_increment())
    }

}
