use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::CuidError;
use crate::text::to_base_string;

pub fn timestamp() -> Result<String, CuidError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // millisecond timestamp to match javascript
        .map(|time| time.as_millis())
        .map(to_base_string)
        .unwrap_or(Err(CuidError::TextError("Could not convert time to str")))
}

#[cfg(test)]
mod time_tests {
    use super::super::BASE;
    use super::*;

    // NOTE: this will start failing in ~2059, at which point this will need to
    // be updated to 9
    #[test]
    fn test_timestamp_len() {
        assert_eq!(timestamp().unwrap().len(), 8);
    }

    #[test]
    fn test_timestamp() {
        assert!(
            (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                - u128::from_str_radix(&timestamp().unwrap(), BASE as u32).unwrap())
                < 5
        )
    }
}
