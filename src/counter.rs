use super::{COUNTER, DISCRETE_VALUES};
use error::CuidError;
use text::to_base_str;

fn fetch_and_increment() -> u32 {
    let mut counter = COUNTER.lock();

    if *counter < (DISCRETE_VALUES - 1) {
        *counter = *counter + 1;
        *counter - 1
    } else {
        *counter = 0;
        DISCRETE_VALUES - 1
    }
}

pub fn current() -> Result<Box<str>, CuidError> {
    to_base_str(fetch_and_increment())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_increasing_basic() {
        {
            let mut counter = COUNTER.lock();
            *counter = 0;
        }

        assert_eq!(0, fetch_and_increment());
        assert_eq!(1, fetch_and_increment());
        assert_eq!(2, fetch_and_increment());
    }

    #[test]
    fn counter_increasing_rollover() {
        {
            let mut counter = COUNTER.lock();
            *counter = DISCRETE_VALUES - 1;
        }
        assert_eq!(DISCRETE_VALUES - 1, fetch_and_increment());
        assert_eq!(0, fetch_and_increment());
    }

    // TODO: Multi-thread counter tests
}
