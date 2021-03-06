//! # cuid-rust
//!
//! CUID generation in rust
//!
//! ```rust
//! use cuid;
//!
//! // Get a full CUID
//! println!("{}", cuid::cuid().unwrap());
//!
//! // Get a shorter, URL-appropriate CUID slug
//! println!("{}", cuid::slug().unwrap());
//! ```
//!
//! Note that this crate also provides a very simple, single purpose
//! commandline interface:
//!
//! ```sh
//! $> cuid
//! ckfritrvg0000kdtwc766fful
//! ```
//!
//! See the [original implementation] for more details on CUIDs in general.
//!
//! [original implementation]: https://github.com/ericelliott/cuid
//!
use std::sync::atomic::AtomicU32;

#[macro_use]
extern crate lazy_static;

mod counter;
mod error;
mod fingerprint;
mod random;
mod text;
mod time;

pub use error::CuidError;

static BASE: u8 = 36;
static BLOCK_SIZE: usize = 4;
static DISCRETE_VALUES: u32 = 1679616; // BASE^BLOCK_SIZE
static START_STR: &str = "c";

static COUNTER: AtomicU32 = AtomicU32::new(0);

lazy_static! {
    static ref FINGERPRINT: String =
        fingerprint::fingerprint().expect("Could not determine system fingerprint!");
}

/// Generate a CUID
///
/// A CUID is composed of:
///
/// - The letter `c`
/// - The timestamp in milliseconds as a base 36 number
/// - An atomic counter that goes from 0 through 36^4 and then repeats, as a
///   base 36 number
/// - A (base 36) fingerprint derived from the system's hostname
/// - Two random numbers between 0 and 36^4, each converted to base 36
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let id = cuid::cuid();
/// assert!(cuid::is_cuid(id.unwrap()));
/// ```
pub fn cuid() -> Result<String, CuidError> {
    Ok([
        START_STR,
        &time::timestamp()?,
        &counter::current()?,
        &FINGERPRINT,
        &random::random_block()?,
        &random::random_block()?,
    ]
    .concat())
}

/// Generate a CUID slug
///
/// CUID slugs are shorter, appropriate for short URLs or other uses
/// where uniqueness is not the primary requirement.
///
/// Note that this library is capable of generating over 2 million CUID slugs
/// per second on a single thread on a fast machine. If your use case involves
/// generating slugs in loops across threads, it is very possible you'll wind up
/// with some non-unique slugs, given that the components of the slug are:
///
/// - Two characters from the millisecond timestamp as base 36
/// - Four characters from the atomic counter, which has only ~1.6 million
///   unique values, as base 36
/// - The first and last character of the fingerprint (which is always the same
///   on a given host)
/// - Two characters from a random block
///
/// For most use cases (i.e. generating fewer than a million slugs per second on
/// a given host), slugs are very likely to be globally unique. However, please
/// bear that limitation in mind, and use full CUIDs if you need a stronger
/// guarantee of uniqueness.
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let slug = cuid::slug();
/// assert!(cuid::is_slug(slug.unwrap()));
/// ```
pub fn slug() -> Result<String, CuidError> {
    let timestamp = time::timestamp()?;
    let count = counter::current()?;
    let rand = random::random_block()?;
    Ok([
        &timestamp[timestamp.len() - 2..],
        &count[count.len().saturating_sub(4)..],
        &FINGERPRINT[..1],
        &FINGERPRINT[FINGERPRINT.len() - 1..],
        &rand[rand.len() - 2..],
    ]
    .concat())
}

/// Return whether a string is a legitimate CUID
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let id = cuid::cuid().unwrap();
/// assert!(cuid::is_cuid(id));
/// ```
pub fn is_cuid<S: Into<String>>(to_check: S) -> bool {
    &to_check.into()[..1] == START_STR
}

/// Return whether a string is a legitimate CUID slug
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let slug = cuid::slug().unwrap();
/// assert!(cuid::is_slug(slug));
/// ```
pub fn is_slug<S: Into<String>>(to_check: S) -> bool {
    let length = to_check.into().len();
    (7..=10).contains(&length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_discrete_values() {
        assert_eq!((BASE as u32).pow(BLOCK_SIZE as u32), DISCRETE_VALUES);
    }

    #[test]
    fn cuid_len() {
        assert_eq!(cuid().unwrap().len(), 25);
    }

    #[test]
    fn cuid_is_cuid() {
        assert!(is_cuid(cuid().unwrap()));
    }

    #[test]
    fn slug_max_len() {
        assert!(slug().unwrap().len() <= 10);
    }

    #[test]
    fn slug_min_len() {
        assert!(slug().unwrap().len() >= 7);
    }

    #[test]
    fn slug_is_slug() {
        assert!(is_slug(slug().unwrap()));
    }
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_cuid(b: &mut Bencher) {
        b.iter(|| {
            cuid().unwrap();
        })
    }

    #[bench]
    fn bench_slug(b: &mut Bencher) {
        b.iter(|| {
            slug().unwrap();
        })
    }
}
