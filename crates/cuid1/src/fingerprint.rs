use std::process;

use crate::text::{pad, to_base_string};
use crate::BASE;

static FINGERPRINT_PADDING: usize = 2;

fn pid() -> String {
    pad(FINGERPRINT_PADDING, to_base_string(process::id()))
}

/// Convert the hostname to a padded String in the appropriate base.
///
/// First converts the hostname to a number comprising the length of the
/// hostname, added to the base radix for CUID strings, added to the sum of
/// the integer value of each character in the hostname, then converts that
/// number to base radix.
fn convert_hostname(hn: &str) -> String {
    pad(
        FINGERPRINT_PADDING,
        to_base_string(
            hn.chars()
                .fold(hn.len() + BASE as usize, |acc, c| acc + c as usize) as u64,
        ),
    )
}

#[cfg(target_family = "wasm")]
/// Wasm doesn't support hostname, so just use a UUID
fn host_id() -> String {
    let hn = uuid::Uuid::new_v4().to_string();
    convert_hostname(&hn)
}

#[cfg(not(target_family = "wasm"))]
fn host_id() -> String {
    // If we can't get a hostname, fall back to a UUID
    let hn = hostname::get()
        .map(|hn| hn.to_string_lossy().to_string())
        .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
    convert_hostname(&hn)
}

pub fn fingerprint() -> String {
    let mut hid = host_id();
    let procid = pid();
    hid.push_str(&procid);
    hid
}

#[cfg(test)]
mod fingerprint_tests {

    use super::*;

    #[test]
    fn test_pid_length() {
        assert_eq!(pid().len(), FINGERPRINT_PADDING)
    }

    // The below expected host_ids were all generated directly using
    // the original javascript implementation.

    #[test]
    fn test_convert_hostname_1() {
        assert_eq!("a3", &*convert_hostname("foo"))
    }

    #[test]
    fn test_convert_hostname_2() {
        assert_eq!("9o", &*convert_hostname("bar"))
    }

    #[test]
    fn test_convert_hostname_3() {
        assert_eq!("nf", &*convert_hostname("mr-magoo"))
    }

    #[test]
    fn test_convert_hostname_4() {
        assert_eq!(
            "j9",
            &*convert_hostname("wow-what-a-long-hostname-you-have")
        )
    }

    #[test]
    fn fingerprint_len() {
        assert_eq!(4, fingerprint().len())
    }
}
