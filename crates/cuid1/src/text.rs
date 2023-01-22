use std::iter;
use std::{char, cmp::Ordering};

use crate::error::CuidError;

pub fn to_base_string<N: Into<u128>>(number: N) -> Result<String, CuidError> {
    Ok(cuid_util::to_base_36(number))
}

/// Pad a string `to_pad` up to size `size` with char `char`
///
/// Inserts `char` at the beginning of the string until the string is equal to
/// size `size`. If `to_pad` is longer than `size`, remove characters from the
/// start of the string until it is of size `size`.
fn pad_with_char(pad_char: char, size: usize, mut to_pad: String) -> String {
    let length = to_pad.len();

    match length.cmp(&size) {
        Ordering::Less => {}
        Ordering::Equal => return to_pad,
        Ordering::Greater => {
            // Cut from the start of the string to pad down to the expected size,
            // e.g. for a size of 2, `abc` would become `bc`
            // benchmarking has shown that returning a new string here is faster
            // than mutating the old one
            return to_pad[length - size..].to_string();
        }
    }

    let size_diff = size - length;

    let to_insert: String = iter::once(pad_char).cycle().take(size_diff).collect();
    to_pad.insert_str(0, &to_insert);
    to_pad
}

pub fn pad(size: usize, to_pad: String) -> String {
    pad_with_char('0', size, to_pad)
}

#[cfg(test)]
mod pad_tests {
    use super::*;

    #[test]
    fn does_not_pad_str_of_size() {
        assert_eq!("foo", &*pad_with_char('a', 3, "foo".into()))
    }

    #[test]
    fn single_char_pad() {
        assert_eq!("afoo", &*pad_with_char('a', 4, "foo".into()))
    }

    #[test]
    fn multichar_pad() {
        assert_eq!("aaafoo", &*pad_with_char('a', 6, "foo".into()))
    }

    #[test]
    fn smaller_pad() {
        assert_eq!("c", &*pad_with_char('a', 1, "abc".into()))
    }

    #[test]
    fn pad_0s() {
        assert_eq!("00foo", &*pad(5, "foo".into()))
    }
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::u32;
    use test::Bencher;

    #[bench]
    fn digits_in_some_base(b: &mut Bencher) {
        b.iter(|| {
            digits_in_base(16, 12345678);
        });
    }

    #[bench]
    fn to_radix_str_less_than_radix(b: &mut Bencher) {
        b.iter(|| {
            to_radix_str(16, 10u8).unwrap();
        });
    }

    #[bench]
    fn to_radix_str_medium(b: &mut Bencher) {
        b.iter(|| {
            to_radix_str(16, 1_000_000_000u32).unwrap();
        });
    }

    #[bench]
    fn to_radix_str_large(b: &mut Bencher) {
        b.iter(|| {
            to_radix_str(16, u32::MAX).unwrap();
        });
    }

    #[bench]
    fn pad_equal(b: &mut Bencher) {
        b.iter(|| {
            pad_with_char('0', 12, "ooo ooo ooo ");
        });
    }

    #[bench]
    fn pad_small_string(b: &mut Bencher) {
        b.iter(|| {
            pad_with_char('0', 12, "oo");
        });
    }

    #[bench]
    fn pad_larger_string(b: &mut Bencher) {
        b.iter(|| {
            pad_with_char('0', 12, "ooo ooo ooo ooo ");
        });
    }
}
