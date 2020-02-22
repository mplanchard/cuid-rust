use crate::error::CuidError;
use crate::text::to_base_string;
use crate::{COUNTER, DISCRETE_VALUES};


/// Fetch the counter value and increment it.
///
/// If the counter has reached its max (DISCRETE VALUES), reset it to 0.
fn fetch_and_increment() -> Result<u32, CuidError> {
    let mut counter = COUNTER.lock().map_err(|_| CuidError::CounterError)?;
    let current = *counter;
    if current == DISCRETE_VALUES - 1 {
        *counter = 0;
    } else {
        *counter += 1;
    };
    Ok(current)
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
        let start = 0;
        {
            let mut counter = COUNTER.lock().unwrap();
            *counter = start;
        }
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
    fn counter_increasing_rollover() {
        let max = DISCRETE_VALUES - 1;
        {
            let mut counter = COUNTER.lock().unwrap();
            *counter = max;
        }
        // Tests run in parallel, so we're not necessarily guaranteed
        // consistent ordering in the context of a test.
        fetch_and_increment().unwrap();  // will return the max unless another thread rolled us over
        let rolled_over = fetch_and_increment().unwrap();  // must be rolled over

        assert!(rolled_over < max);
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
