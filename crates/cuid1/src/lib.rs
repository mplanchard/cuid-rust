//! CUID version one

mod counter;
mod fingerprint;
mod random;
mod text;
mod time;

use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicU32, Ordering};

const BASE: u8 = 36;
const BLOCK_SIZE: usize = 4;
const DISCRETE_VALUES: u32 = 1679616; // BASE^BLOCK_SIZE
const START_STR: &str = "c";

static COUNTER: AtomicU32 = AtomicU32::new(0);

static FINGERPRINT: Lazy<String> = Lazy::new(fingerprint::fingerprint);

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
/// let id = cuid1::cuid();
/// assert!(cuid1::is_cuid(id));
/// ```
#[inline]
pub fn cuid() -> String {
    [
        START_STR,
        &time::timestamp(),
        &counter::current(),
        &FINGERPRINT,
        &random::random_block(),
        &random::random_block(),
    ]
    .concat()
}

/// Generate a single CUID, for use in the cuid binary.
///
/// Sets the counter to a random value before generation, so that it isn't
/// always 0.
#[doc(hidden)]
#[inline]
pub fn one_off_cuid1() -> String {
    let counter_init = thread_rng().gen();
    COUNTER.store(counter_init, Ordering::Relaxed);
    cuid()
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
/// let slug = cuid1::slug();
/// assert!(cuid1::is_slug(slug));
/// ```
#[inline]
pub fn slug() -> String {
    let timestamp = time::timestamp();
    let count = counter::current();
    let rand = random::random_block();
    [
        &timestamp[timestamp.len() - 2..],
        &count[count.len().saturating_sub(4)..],
        &FINGERPRINT[..1],
        &FINGERPRINT[FINGERPRINT.len() - 1..],
        &rand[rand.len() - 2..],
    ]
    .concat()
}

/// Generate a single CUID slug, for use in the cuid binary.
///
/// Sets the counter to a random value before generation, so that it isn't
/// always 0.
#[doc(hidden)]
#[inline]
pub fn one_off_cuid1_slug() -> String {
    let counter_init = thread_rng().gen();
    COUNTER.store(counter_init, Ordering::Relaxed);
    slug()
}

/// Return whether a string looks like it could be a legitimate CUID
///
/// # Examples
///
/// ```rust
/// let id = cuid1::cuid();
/// assert!(cuid1::is_cuid(id));
/// ```
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

/// Return whether a string looks like it could be a legitimate v1 CUID slug
///
/// # Examples
///
/// ```rust
/// let slug = cuid1::slug();
/// assert!(cuid1::is_slug(slug));
/// ```
#[inline]
pub fn is_slug<S: AsRef<str>>(to_check: S) -> bool {
    // the slug will always be 10 characters
    to_check.as_ref().len() == 10
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run an already-defined test in WASM as well.
    macro_rules! wasm_test {
        ($name:ident) => {
            paste::paste! {
                #[wasm_bindgen_test::wasm_bindgen_test]
                fn [<wasm_ $name>]() {
                    $name()
                }
            }
        };
    }

    #[test]
    fn correct_discrete_values() {
        assert_eq!((crate::BASE as u32).pow(BLOCK_SIZE as u32), DISCRETE_VALUES);
    }
    wasm_test!(correct_discrete_values);

    #[test]
    fn cuid_len() {
        assert_eq!(cuid().len(), 25);
    }
    wasm_test!(cuid_len);

    #[test]
    fn cuid_is_cuid() {
        assert!(is_cuid(cuid()));
    }
    wasm_test!(cuid_is_cuid);

    #[test]
    fn cuid_is_not_cuid_zero_len() {
        assert!(!is_cuid(""));
    }
    wasm_test!(cuid_is_not_cuid_zero_len);

    #[test]
    fn slug_len() {
        assert!(slug().len() == 10);
    }
    wasm_test!(slug_len);

    #[test]
    fn slug_is_slug() {
        assert!(is_slug(slug()));
    }
    wasm_test!(slug_is_slug);
}
