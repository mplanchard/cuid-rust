use std::sync::atomic::Ordering;

use arraystring::ArrayString;

use crate::error::CuidError;
use crate::text::{pad, to_base36_string};
use crate::{BlockSize, COUNTER, DISCRETE_VALUES};

/// Fetch the counter value and increment it.
///
/// If the counter has reached its max (DISCRETE VALUES), reset it to 0.
fn fetch_and_increment() -> Result<u32, CuidError> {
    COUNTER
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| match i {
            i if i == DISCRETE_VALUES - 1 => Some(0),
            _ => Some(i + 1),
        })
        .map_err(|_| CuidError::CounterError)
}

/// Return the current counter value in the appropriate base as a String.
pub fn current() -> Result<ArrayString<BlockSize>, CuidError> {
    fetch_and_increment().map(to_base36_string)?.map(|s| pad(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests are ignored because they depend on global state and must be
    // run with --test-threads=1 in order to work.

    #[test]
    #[ignore]
    fn counter_increasing_basic() {
        let start = 0;
        COUNTER.store(start, Ordering::SeqCst);
        // Tests run in parallel, so we're not necessarily guaranteed
        // consistent ordering in the context of a test.
        let first = fetch_and_increment().unwrap();
        assert!(first >= start);
        let second = fetch_and_increment().unwrap();
        assert!(second > first);
        let third = fetch_and_increment().unwrap();
        assert!(third > second);
    }

    #[test]
    #[ignore]
    fn counter_is_monotonic_with_increasing_length() {
        // Ensure that counter is lexigraphically-monotonic even if the counter length in base-36
        // increases (1 digit -> 2 digits).
        let start = 35;
        COUNTER.store(start, Ordering::SeqCst);
        // Tests run in parallel, so we're not necessarily guaranteed
        // consistent ordering in the context of a test.
        // Prefer running tests with: cargo test -- --test-threads=1
        let first = current().unwrap();
        let second = current().unwrap();
        assert!(second > first);
    }

    #[test]
    #[ignore]
    fn counter_increasing_rollover() {
        let max = DISCRETE_VALUES - 1;
        COUNTER.store(max, Ordering::SeqCst);
        // Tests run in parallel, so we're not necessarily guaranteed
        // consistent ordering in the context of a test.
        fetch_and_increment().unwrap(); // will return the max unless another thread rolled us over
        let rolled_over = fetch_and_increment().unwrap(); // must be rolled over

        assert!(rolled_over < max);
    }

    // TODO: Multi-thread counter tests
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn basic_increment(b: &mut Bencher) {
        b.iter(|| fetch_and_increment())
    }
}
