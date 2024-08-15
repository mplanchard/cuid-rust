//! CUID version one (deprecated, maintained for backwards compatibility)

mod counter;
mod fingerprint;
mod random;

use crate::error::CuidError;
use crate::time;

use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicU32, Ordering};

const BLOCK_SIZE: usize = 4;
const DISCRETE_VALUES: u32 = 1679616; // BASE^BLOCK_SIZE
const START_STR: &str = "c";

static COUNTER: AtomicU32 = AtomicU32::new(0);

static FINGERPRINT: Lazy<String> =
    Lazy::new(|| fingerprint::fingerprint().expect("Could not determine system fingerprint!"));

/// Generate a CUID
///
/// Deprecated. Please use [`cuid1()`] or [`cuid2()`][crate::cuid2()] instead.
#[deprecated(
    since = "1.3.0",
    note = "Using cuid() without specifying a version is no longer supported. Please use cuid1() or cuid2() instead."
)]
#[inline]
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

/// Generate a v1 CUID
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
#[inline]
pub fn cuid1() -> Result<String, CuidError> {
    #[allow(deprecated)]
    cuid()
}

/// Generate a single CUID, for use in the cuid binary.
///
/// Sets the counter to a random value before generation, so that it isn't
/// always 0.
#[doc(hidden)]
#[inline]
pub fn one_off_cuid1() -> Result<String, CuidError> {
    let counter_init = thread_rng().gen();
    COUNTER.store(counter_init, Ordering::Relaxed);
    cuid1()
}

/// Generate a CUID slug
///
/// Deprecated. Please use [`cuid1_slug()`] or [`cuid2_slug()`][crate::cuid2_slug()]
/// instead.
#[deprecated(
    since = "1.3.0",
    note = "Using slug() without specifying a version is no longer supported. Please use cuid1_slug() or cuid2_slug() instead."
)]
#[inline]
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

/// Generate a CUID v1 slug
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
#[inline]
pub fn cuid1_slug() -> Result<String, CuidError> {
    #[allow(deprecated)]
    slug()
}

/// Generate a single CUID slug, for use in the cuid binary.
///
/// Sets the counter to a random value before generation, so that it isn't
/// always 0.
#[doc(hidden)]
#[inline]
pub fn one_off_cuid1_slug() -> Result<String, CuidError> {
    let counter_init = thread_rng().gen();
    COUNTER.store(counter_init, Ordering::Relaxed);
    cuid1_slug()
}

/// Return whether a string is a legitimate CUID
///
/// Deprecated. Please use [`is_cuid1()`] or [`is_cuid2()`][crate::is_cuid2()]
/// instead.
#[deprecated(
    since = "1.3.0",
    note = "Using is_cuid() without specifying a version is no longer supported. Please use is_cuid1() instead."
)]
#[inline]
pub fn is_cuid<S: AsRef<str>>(to_check: S) -> bool {
    let to_check = to_check.as_ref();
    match to_check.len() {
        // the CUID length will increase as the timestamp increases. The
        // timestamp portion currently represents 8 characters. It has the
        // potential to increase to up to 15 characters when the timestamp
        // reaches the maximum 64-bit integer, at which point the earth will be
        // long gone, presumably along with this code. At that time, the CUID
        // length would be 32. 9 characters gives us up through at least the
        // year 5138, though, so checking for 25 or 26 characters should do it.
        25..=26 => &to_check[..1] == START_STR,
        _ => false,
    }
}

/// Return whether a string looks like it could be a legitimate CUID
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let id = cuid::cuid().unwrap();
/// assert!(cuid::is_cuid(id));
/// ```
#[inline]
pub fn is_cuid1<S: AsRef<str>>(to_check: S) -> bool {
    #[allow(deprecated)]
    is_cuid(to_check)
}

/// Return whether a string looks like it could be a legitimate CUID slug
///
/// Deprecated. Please use [`is_cuid1_slug()`] or [`is_cuid2_slug()`][crate::is_cuid2_slug()]
/// instead.
#[deprecated(
    since = "1.3.0",
    note = "Using is_slug() without specifying a version is no longer supported. Please use is_cuid1_slug() or is_cuid2_slug() instead."
)]
#[inline]
pub fn is_slug<S: AsRef<str>>(to_check: S) -> bool {
    // the slug will always be 10 characters
    to_check.as_ref().len() == 10
}

/// Return whether a string looks like it could be a legitimate v1 CUID slug
///
/// # Examples
///
/// ```rust
/// extern crate cuid;
/// let slug = cuid::slug().unwrap();
/// assert!(cuid::is_slug(slug));
/// ```
#[inline]
pub fn is_cuid1_slug<S: AsRef<str>>(to_check: S) -> bool {
    #[allow(deprecated)]
    is_slug(to_check)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_discrete_values() {
        assert_eq!((crate::BASE as u32).pow(BLOCK_SIZE as u32), DISCRETE_VALUES);
    }

    #[test]
    fn cuid_len() {
        assert_eq!(cuid1().unwrap().len(), 25);
    }

    #[test]
    fn cuid_is_cuid() {
        assert!(is_cuid1(cuid1().unwrap()));
    }

    #[test]
    fn cuid_is_not_cuid_zero_len() {
        assert!(!is_cuid1(""));
    }

    #[test]
    fn slug_len() {
        assert!(cuid1_slug().unwrap().len() == 10);
    }

    #[test]
    fn slug_is_slug() {
        assert!(is_cuid1_slug(cuid1_slug().unwrap()));
    }
}
