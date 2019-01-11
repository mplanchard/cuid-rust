
use std::time::{SystemTime, UNIX_EPOCH};

use text::to_base_str;


pub fn timestamp() -> String {
    to_base_str(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    )
}


#[cfg(tests)]
mod time_tests {
    use super::*;
    use super::super::BASE;

    #[test]
    fn test_timestamp() {
        assert!(
            (
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                - u64::from_str_radix(timestamp(), BASE)
            ) < 5
        )
    }
}
