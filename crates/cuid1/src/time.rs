// std::time::SystemTime panics on WASM, so use a different library there.
#[cfg(not(target_family = "wasm"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_family = "wasm")]
use web_time::{SystemTime, UNIX_EPOCH};

use crate::text::to_base_string;

pub fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // millisecond timestamp to match javascript
        .map(|time| time.as_millis())
        .map(to_base_string)
        .expect(
            "Failed to calculate system timestamp! Current system time may be \
                 set to before the Unix epoch, or time may otherwise be broken. \
                 Cannot continue",
        )
}

#[cfg(test)]
mod time_tests {
    use super::super::BASE;
    use super::*;

    // NOTE: this will start failing in ~2059, at which point this will need to
    // be updated to 9
    #[test]
    fn test_timestamp_len() {
        assert_eq!(timestamp().len(), 8);
    }

    #[test]
    fn test_timestamp() {
        assert!(
            (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                - u128::from_str_radix(&timestamp(), BASE as u32).unwrap())
                < 5
        )
    }
}
