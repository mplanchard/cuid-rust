//! # Cuid2
//!
//! Secure, collision-resistant ids optimized for horizontal scaling and
//! performance.
//!
//! This is a Rust implementation of the CUID2 algorithm, defined by its
//! reference implementation [here](https://github.com/paralleldrive/cuid2).
//!
//! Please see that repository for a discussion of the benefits of CUIDs, as
//! well as for the improvements in CUID2 over the original CUID algorithm
//! (which is also implemented in Rust [here](https://docs.rs/cuid/latest/cuid/)).
//!
//! ## Usage
//!
//! The simplest usage is to use the `create_id()` function to create an ID:
//!
//! ```
//! use cuid2;
//!
//! let id = cuid2::create_id();
//!
//! assert_eq!(24, id.len());
//! ```
//!
//! A `cuid()` alias is provided to make this more of a drop-in replacement for
//! the v1 cuid package:
//!
//! ```
//! use cuid2::cuid;
//!
//! let id = cuid();
//!
//! assert_eq!(24, id.len());
//! ```
//!
//! If you would like to customize aspects of CUID production, you can create
//! a constructor with customized properties:
//!
//! ```
//! use cuid2::CuidConstructor;
//!
//! let constructor = CuidConstructor::new().with_length(32);
//!
//! let id = constructor.create_id();
//!
//! assert_eq!(32, id.len());
//! ```
//!
//! If installed with `cargo install`, this package also provides a `cuid2`
//! binary, which generates a CUID on the command line. It can be used like:
//!
//! ```ignore,compile_fail
//! > cuid2
//! y3cfw1hafbtezzflns334sb2
//! ```

use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[cfg(not(target_family = "wasm"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_family = "wasm")]
use web_time::{SystemTime, UNIX_EPOCH};

use cuid_util::ToBase36;
use num::bigint;
use rand::{seq::SliceRandom, thread_rng, Rng};
use sha3::{Digest, Sha3_512};

// =============================================================================
// CONSTANTS
// =============================================================================

// Only public to expose to binary
#[doc(hidden)]
pub const DEFAULT_LENGTH: u8 = 24;
const BIG_LENGTH: u8 = 32;
// valid characters to start an ID
const STARTING_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

// =============================================================================
// THREAD LOCALS
// =============================================================================
// Each thread generating CUIDs gets its own:
// - 64-bit counter, randomly initialized to some value between 0 and 2056, inclusive
// - fingerprint, a hash with added entropy, derived from a random number between
//   2063 and 4125, inclusive, the process ID, and the thread ID

fn fingerprint() -> String {
    hash(
        [
            thread_rng().gen::<u128>().to_be_bytes(),
            thread_rng().gen::<u128>().to_be_bytes(),
            #[cfg(not(target_family = "wasm"))]
            u128::from(std::process::id()).to_be_bytes(),
            #[cfg(target_family = "wasm")]
            thread_rng().gen::<u128>().to_be_bytes(),
            u128::from(get_thread_id()).to_be_bytes(),
        ],
        BIG_LENGTH.into(),
    )
}

thread_local! {
    /// Value used to initialize the counter. After the counter hits u64::MAX, it
    /// will roll back to this value.
    // Updated 2023-08-08 to match updated reference implementation, which notes:
    // > ~22k hosts before 50% chance of initial counter collision
    // > with a remaining counter range of 9.0e+15 in JavaScript.
    static COUNTER_INIT: u64 = thread_rng().gen_range(0..476_782_367);

    /// Use an individual counter per thread, starting at a randomly initialized value.
    ///
    /// Range of randomly initialized values taken from reference implementation.
    static COUNTER: RefCell<u64> = COUNTER_INIT.with(|val| RefCell::new(*val));

    /// Fingerprint! The original implementation is a hash of:
    /// - stringified keys of the global object
    /// - added entropy
    ///
    /// For us, we'll use
    /// - A few random numbers
    /// - the process ID
    /// - the thread ID (which also ensures our CUIDs will be different per thread)
    ///
    /// This is pretty non-language, non-system dependent, so it allows us to
    /// compile to wasm and so on.
    static FINGERPRINT: String = fingerprint();
}

// Hashing
// =======

/// Hash a value, including an additional salt of randomly generated data.
//
// Updated 2023-08-08 to match the updated JS implementation, which is:
//
// ```js
// const hash = (input = "") => {
//   // Drop the first character because it will bias the histogram
//   // to the left.
//   return bufToBigInt(sha3(input)).toString(36).slice(1);
// };
// ```
//
// We don't drop the first character, because it doesn't actually affect the
// histogram (the comment in the reference implementation is incorrect).
fn hash<S: AsRef<[u8]>, T: IntoIterator<Item = S>>(input: T, length: u16) -> String {
    let mut hasher = Sha3_512::new();

    for block in input {
        hasher.update(block.as_ref());
    }

    // 512 bits (64 bytes) of data ([u8; 64])
    let hash = hasher.finalize();

    // We'll convert the bytes directly to a big, unsigned int and then use
    // its builtin radix conversion.
    //
    // We don't use bigint for the rest of our base conversions, because it's
    // significantly slower, but we use it here since we need to deal with the
    // 512-bit integer from the hash function.
    let mut res = bigint::BigUint::from_bytes_be(&hash).to_str_radix(36);

    // Note that truncate panics if the length does not fall on a char boundary,
    // but we don't need to worry about that since all the chars will be ASCII.
    res.truncate(length.into());

    res
}

// Other Utility Functions
// =======================

/// Return whether a string is a legitimate CUID2
/// ```rust
/// use cuid2;
/// let id = cuid2::create_id();
/// let empty_id = "";
/// let too_small = "a";
/// let too_big = "a1l23j1l2k3j12o8312j3k12j3lj12k3j1lk2j312j3lkj12l3g1kj2h312312lk3j1l2j3lk12j3lkjlj1lk23jl131l2k3jl12j3lk1j2lk3j12lk3h12k3hhl1j2j3";
/// let non_ascii_alphanumeric = "a#";
/// let non_first_letter = "1aaa";
/// let with_underscore = "aaa_1aaa";
/// assert!(cuid2::is_cuid2(id));
/// assert!(!cuid2::is_cuid2(empty_id));
/// assert!(!cuid2::is_cuid2(too_small));
/// assert!(!cuid2::is_cuid2(too_big));
/// assert!(!cuid2::is_cuid2(non_ascii_alphanumeric));
/// assert!(!cuid2::is_cuid2(non_first_letter));
/// assert!(!cuid2::is_cuid2(with_underscore));
/// ```
#[inline]
pub fn is_cuid2<S: AsRef<str>>(to_check: S) -> bool {
    const MAX_LENGTH: usize = BIG_LENGTH as usize;
    is_cuid2_inner::<S, MAX_LENGTH>(to_check)
}

fn is_cuid2_inner<S: AsRef<str>, const MAX_LENGTH: usize>(to_check: S) -> bool {
    let to_check = to_check.as_ref().as_bytes();

    if (2..=MAX_LENGTH).contains(&to_check.len()) {
        if let [first, tail @ ..] = to_check {
            return STARTING_CHARS.as_bytes().contains(first)
                && tail.iter().all(|x| matches!(x, b'0'..=b'9' | b'a'..=b'z'));
        }
    }

    false
}

/// Return whether a string is a legitimate CUID.
///
/// This is an alias of [is_cuid2]
#[inline]
pub fn is_cuid<S: AsRef<str>>(to_check: S) -> bool {
    is_cuid2(to_check)
}

/// Creates a random string of the specified length.
fn create_entropy(length: u16) -> String {
    let mut rng = thread_rng();
    let length: usize = length.into();

    // Allocate a string with the appropriate capacity to avoid reallocation.
    //
    // The string is generated and then pushed to until its desired length is
    // reached or exceeded. We therefore allocate enough for the length plus
    // the maximum value it might be exceeded by. The values pushed to the
    // string are random numbers from 0 to 36, converted to base 36.
    // Therefore, the maximum overfill is 36 in base 36, i.e. 10, which is 2
    // chars
    let mut result = String::with_capacity(length + 2);

    while result.len() < length {
        // Matches reference implementation logic as of 2023-08-08, which is:
        // ```js
        // entropy = entropy + Math.floor(random() * 36).toString(36);
        // ```
        let random_val = rng.gen_range(0u128..36u128);
        result.push_str(&random_val.to_base_36());
    }

    result
}

/// Retrieves the current timestmap and converts to Base36.
fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // Use timestamp as milliseconds to match JS implementation
        .map(|time| time.as_millis().to_base_36())
        // Panic safety: `.duration_since()` fails if the end time is not
        // later than the start time, so this will only fail if the system
        // time is before 1970-01-01. It is impossible on Unix systems to set
        // a time before then, since the entire system uses a 32 or 64 bit
        // unsigned integer for time, where zero is midnight 1970-01-01.
        //
        // If you are on a system that for some reason both can be and needs to
        // be set >50 years in the past AND this library not working is a
        // problem for you, please feel free to reach out.
        .expect(
            "Failed to calculate system timestamp! Current system time may be \
                 set to before the Unix epoch, or time may otherwise be broken. \
                 Cannot continue",
        )
}

/// Retrieves and increments the counter value.
fn get_count() -> u64 {
    COUNTER.with(|cell| {
        cell.replace_with(|counter| {
            counter
                .checked_add(1)
                // if we hit u64::MAX, roll back to the original thread-local
                // initialization value
                .unwrap_or_else(|| COUNTER_INIT.with(|x| *x))
        })
    })
}

/// Retrieves the thread-local fingerprint.
fn get_fingerprint() -> String {
    FINGERPRINT.with(|x| x.clone())
}

/// Retrieves the current thread's ID.
fn get_thread_id() -> u64 {
    // ThreadId doesn't implement debug or display, but it does implement Hash,
    // so we can get the hash value to use in our fingerprint.
    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

// =============================================================================
// CUID CONSTRUCTION
// =============================================================================

/// Provides customization of CUID generation.
///
/// ```
/// use cuid2::CuidConstructor;
///
/// let mut constructor = CuidConstructor::new();
/// assert_eq!(24, constructor.create_id().len());
///
/// constructor.set_length(16);
/// assert_eq!(16, constructor.create_id().len());
///
/// assert_eq!(32, CuidConstructor::new().with_length(32).create_id().len());
/// ```
pub struct CuidConstructor {
    length: u16,
    counter: fn() -> u64,
    fingerprinter: fn() -> String,
}
impl CuidConstructor {
    /// Creates a new constructor with default settings.
    pub const fn new() -> Self {
        Self {
            length: DEFAULT_LENGTH as u16,
            counter: get_count,
            fingerprinter: get_fingerprint,
        }
    }

    /// Returns a new constructor that will generate CUIDs with the specified length.
    ///
    /// # Panics
    ///
    /// Panics if `length` is less than 2.
    ///
    pub const fn with_length(self, length: u16) -> Self {
        if length < 2 {
            panic!("CUID length must be at least 2")
        }
        Self { length, ..self }
    }

    /// Returns a new constructor with the specified counter function.
    pub const fn with_counter(self, counter: fn() -> u64) -> Self {
        Self { counter, ..self }
    }

    /// Returns a new constructor with the specified fingerprinter function.
    pub const fn with_fingerprinter(self, fingerprinter: fn() -> String) -> Self {
        Self {
            fingerprinter,
            ..self
        }
    }

    /// Sets the length for CUIDs generated by this constrctor.
    ///
    /// # Panics
    ///
    /// Panics if `length` is less than 2.
    ///
    pub fn set_length(&mut self, length: u16) {
        if length < 2 {
            panic!("CUID length must be at least 2")
        }
        self.length = length;
    }

    /// Sets the counter function for this constructor.
    pub fn set_counter(&mut self, counter: fn() -> u64) {
        self.counter = counter;
    }

    /// Sets the fingerperinter function for this constructor.
    pub fn set_fingerprinter(&mut self, fingerprinter: fn() -> String) {
        self.fingerprinter = fingerprinter;
    }

    /// Creates a new CUID.
    #[inline]
    pub fn create_id(&self) -> String {
        let time = get_timestamp();

        let entropy = create_entropy(self.length);

        let count = (self.counter)().to_base_36();

        let fingerprint = (self.fingerprinter)();

        // Construct the main part of the ID body by hashing the various inputs
        let id_body = hash(
            [
                time.as_bytes(),
                entropy.as_bytes(),
                count.as_bytes(),
                fingerprint.as_bytes(),
            ],
            // The hash should be the desired total length minus 1 character
            // for the starting char.
            self.length - 1,
        );

        // TODO check if index access makes a perf difference here
        let first_letter = (*STARTING_CHARS
            .as_bytes()
            // Panic safety: choose() only returns None if the slice is empty,
            // and STARTING_CHARS is a statically defined non-empty slice.
            .choose(&mut thread_rng())
            .expect("STARTING_CHARS cannot be empty")) as char;

        // Return only the requested length
        format!("{first_letter}{id_body}")
    }
}
impl Default for CuidConstructor {
    fn default() -> Self {
        Self::new()
    }
}

/// Use a static constructor for create_id() so that we don't need to pay the
/// (minimal, probably trivial) cost of constructor creation when called via
/// `create_id()`.
static DEFAULT_CONSTRUCTOR: CuidConstructor = CuidConstructor::new();

const SLUG_LENGTH: u16 = 10;

static SLUG_CONSTRUCTOR: CuidConstructor = CuidConstructor::new().with_length(SLUG_LENGTH);

/// Creates a new CUID.
#[inline]
pub fn create_id() -> String {
    DEFAULT_CONSTRUCTOR.create_id()
}

/// Creates a new CUID.
///
/// Alias for `created_id()`, which is the interface defined in the reference
/// implementation. The `cuid()` interface allows easier drop-in replacement
/// for crates using the v1 `cuid` crate.
#[inline]
pub fn cuid() -> String {
    create_id()
}

/// Creates a new CUID slug, which is just a CUID with a length of 10 characters.
#[inline]
pub fn slug() -> String {
    SLUG_CONSTRUCTOR.create_id()
}

/// Return whether a string looks like it could be a legitimate CUID slug.
#[inline]
pub fn is_slug<S: AsRef<str>>(to_check: S) -> bool {
    const MAX_LENGTH: usize = SLUG_LENGTH as usize;
    is_cuid2_inner::<S, MAX_LENGTH>(to_check)
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, thread};

    use super::*;

    #[test]
    fn counter_increments() {
        let start = get_count();
        let next = get_count();

        // concurrent test may have also incremented
        assert!(next > start);
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn wasm_cuid_does_not_panic() {
        cuid();
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    #[ignore] // slow: run explicitly when desired
    fn collisions() {
        // generate ~10e6 IDs across all available cores
        let cores = num_cpus::get();
        let per_core = 10_000_000 / cores;

        // collect to force spawning the threads instead of just holding them lazily
        #[allow(clippy::needless_collect)]
        let threads = (0..cores)
            .map(|_| thread::spawn(move || (0..per_core).map(|_| create_id()).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let res = threads
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>();

        // All IDs are unique
        assert_eq!(res.iter().collect::<HashSet<_>>().len(), res.len())
    }

    /// Asserts that CUIDs are uniformly distributed, ignoring the first
    /// character.
    ///
    /// See https://github.com/paralleldrive/cuid2/blob/b5665387fdf7f947e135f030a545df22c5010a7d/src/test-utils.js
    /// and https://github.com/paralleldrive/cuid2/blob/b5665387fdf7f947e135f030a545df22c5010a7d/src/histogram.js
    #[test]
    #[ignore] // slow: run explicitly when desired
    fn distribution() {
        let count = 1_000_000;

        let buckets = [0_u64; 20];
        let bucket_count = bigint::BigUint::from(buckets.len());

        let histogram = (0..count)
            .map(|_| create_id())
            // parse the ID (minus starting char) as a base36 number
            .map(|id| bigint::BigUint::parse_bytes(id[1..].as_bytes(), 36).unwrap())
            // Determine its bucket number.
            // We know the bucket number will be <20, so we .to_u32_digits()
            // and grab what should be the only item.
            .map(|num| {
                let bucket_number = &num % &bucket_count;
                let digits = bucket_number.to_u32_digits();
                assert!(digits.len() < 2, "{num}: {bucket_number}: {digits:?}");
                digits.first().copied().unwrap_or(0)
            })
            // create the histogram. For each bucket number, increment the count
            .fold(buckets, |mut histogram, bucket_num| {
                histogram[bucket_num as usize] += 1;
                histogram
            });

        let expected_bucket_size = count / histogram.len();
        let tolerance = 0.05;
        let max_bucket_size = (expected_bucket_size as f64 * (1.0 + tolerance)).round() as u64;
        let min_bucket_size = (expected_bucket_size as f64 * (1.0 - tolerance)).round() as u64;

        histogram
            .into_iter()
            .enumerate()
            .for_each(|(idx, bucket_size)| {
                assert!(bucket_size > min_bucket_size, "bucket {idx} too small");
                assert!(bucket_size < max_bucket_size, "bucket {idx} too big");
            })
    }
}
