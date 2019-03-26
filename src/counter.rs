use super::{BLOCK_SIZE, COUNTER, DISCRETE_VALUES};

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

pub fn current() -> String {
    format!(
        "{:0width$}",
        fetch_and_increment(),
        width = BLOCK_SIZE as usize
    )
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
