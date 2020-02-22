
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::CuidError;
use crate::text::to_base_string;


pub fn timestamp() -> Result<String, CuidError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|time| time.as_secs())
        .map(to_base_string)
        .unwrap_or(Err(CuidError::TextError("Could not convert time to str")))
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
