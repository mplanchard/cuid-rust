use std::sync::atomic::Ordering;
use super::{ BASE, BLOCK_SIZE, COUNTER, DISCRETE_VALUES };


pub fn fetch_and_increment() -> usize {
    COUNTER.fetch_update(
        |c| { if c < DISCRETE_VALUES - 1 { Some(c + 1) } else { Some(0) } },
        Ordering::SeqCst,
        Ordering::SeqCst,
    ).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_discrete_values() {
        assert_eq!(
            (BASE as usize).pow(BLOCK_SIZE as u32),
            DISCRETE_VALUES,
        );
    }

    #[test]
    fn counter_increasing_basic() {
        assert_eq!(0, fetch_and_increment());
        assert_eq!(1, fetch_and_increment());
        assert_eq!(2, fetch_and_increment());
    }

    #[test]
    fn counter_increasing_rollover() {
        COUNTER.store(DISCRETE_VALUES - 1, Ordering::SeqCst);
        assert_eq!(DISCRETE_VALUES - 1, fetch_and_increment());
        assert_eq!(0, fetch_and_increment());
    }

    // TODO: Multi-thread counter tests
}
