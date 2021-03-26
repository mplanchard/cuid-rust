use std::process;

use hostname;

use crate::error::CuidError;
use crate::text::{pad, to_base_string};
use crate::BASE;

static FINGERPRINT_PADDING: usize = 2;

fn pid() -> Result<String, CuidError> {
    to_base_string(process::id())
        .map(|mut s| pad(FINGERPRINT_PADDING, s))
        .map_err(|_| CuidError::FingerprintError("Could not encode pid"))
}

/// Convert the hostname to a padded String in the appropriate base
fn convert_hostname(hn: &String) -> Result<String, CuidError> {
    to_base_string(
        hn.chars()
            .fold(hn.len() + BASE as usize, |acc, c| acc + c as usize) as u64,
    )
    .map(|mut base_str| pad(FINGERPRINT_PADDING, base_str))
}

fn host_id() -> Result<String, CuidError> {
    let hn = hostname::get()?;
    convert_hostname(&hn.into_string()?)
}

pub fn fingerprint() -> Result<String, CuidError> {
    let hid = host_id()?;
    let procid = pid()?;
    Ok([procid, hid].concat())
}

#[cfg(test)]
mod fingerprint_tests {

    use super::*;

    #[test]
    fn test_pid_length() {
        assert_eq!(pid().unwrap().len(), FINGERPRINT_PADDING)
    }

    // The below expected host_ids were all generated directly using
    // the original javascript implementation.

    #[test]
    fn test_convert_hostname_1() {
        assert_eq!("a3", &*convert_hostname(&"foo".into()).unwrap())
    }

    #[test]
    fn test_convert_hostname_2() {
        assert_eq!("9o", &*convert_hostname(&"bar".into()).unwrap())
    }

    #[test]
    fn test_convert_hostname_3() {
        assert_eq!("nf", &*convert_hostname(&"mr-magoo".into()).unwrap())
    }

    #[test]
    fn test_convert_hostname_4() {
        assert_eq!(
            "j9",
            &*convert_hostname(&"wow-what-a-long-hostname-you-have".into()).unwrap()
        )
    }

    #[test]
    fn fingerprint_len() {
        assert_eq!(4, fingerprint().unwrap().len())
    }
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_pid(b: &mut Bencher) {
        b.iter(|| {
            pid().unwrap();
        })
    }

    #[bench]
    fn bench_convert_hostname_real(b: &mut Bencher) {
        b.iter(|| {
            convert_hostname(get_hostname).unwrap();
        })
    }

    #[bench]
    fn bench_convert_hostname_mock(b: &mut Bencher) {
        b.iter(|| {
            convert_hostname(|| Some(String::from("hostname"))).unwrap();
        })
    }

    #[bench]
    fn bench_fingerprint(b: &mut Bencher) {
        b.iter(|| {
            fingerprint().unwrap();
        })
    }
}
