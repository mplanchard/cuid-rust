use std::{char, cmp::Ordering};
use std::{f64, iter};

use arraystring::prelude::Capacity;
use arraystring::typenum::{Integer, NonZero, Unsigned, N25, U19, U23, U4};
use arraystring::ArrayString;

use crate::error::CuidError;
use crate::BASE;

/// Convert a number to a base 36 string
pub fn to_base36_string<N: Into<u128>>(number: N) -> Result<ArrayString<U23>, CuidError> {
    let mut number = number.into();
    let rad_u32: u32 = BASE.into();

    if number < BASE.into() {
        // No need to allocate a vector or do any math
        // NOTE: we are okay to cast to u32 here, b/c number < radix,
        // which has to be 255 or below.
        return char::from_digit(number as u32, rad_u32)
            .map(|c| {
                let mut s = ArrayString::<U23>::new();
                s.try_push(c).expect("definitely fits");
                s
            })
            .ok_or(CuidError::TextError("Bad digit"));
    } else if number > f64::MAX as u128 {
        return Err(CuidError::TextError("Input number too large"));
    }

    // 32 chars should almost always be enough to fill without needing to grow
    // let mut chars: Vec<char> = Vec::with_capacity(25);
    let mut rv = ArrayString::<U23>::new();
    while number > 0 {
        // We can unwrap here b/c we know that the modulus must be less than the
        // radix, which is less than 256
        // chars.push(char::from_digit((number % BASE as u128) as u32, rad_u32).unwrap());
        rv.try_insert(
            0,
            char::from_digit((number % BASE as u128) as u32, rad_u32).unwrap(),
        )
        .expect("could not insert");
        number /= BASE as u128;
    }
    Ok(rv)
    // chars.reverse();
    // Ok(chars.into_iter().collect())
}

/// Pad a string `to_pad` up to size `size` with char `char`
///
/// Inserts `char` at the beginning of the string until the string is equal to
/// size `size`. If `to_pad` is longer than `size`, remove characters from the
/// start of the string until it is of size `size`.
fn pad_with_char<T: Unsigned + Capacity>(
    pad_char: char,
    mut to_pad: ArrayString<U23>,
) -> ArrayString<T> {
    let size = <T as Unsigned>::to_usize();
    let length = to_pad.as_str().len();

    match length.cmp(&size) {
        Ordering::Less => {}
        Ordering::Equal => return ArrayString::<T>::try_from_str(to_pad).unwrap(),
        Ordering::Greater => {
            // Cut from the start of the string to pad down to the expected size,
            // e.g. for a size of 2, `abc` would become `bc`
            // benchmarking has shown that returning a new string here is faster
            // than mutating the old one
            return ArrayString::try_from_utf8(&to_pad.as_str()[length - size..])
                .expect("size has been validated");
        }
    }

    let size_diff = size - length;

    let to_insert: ArrayString<T> = iter::once(pad_char).cycle().take(size_diff).collect();
    to_pad
        .insert_str(0, &to_insert)
        .expect("could not insert str");
    ArrayString::<T>::try_from_str(to_pad).unwrap()
    // to_pad
}

pub fn pad<T: Unsigned + Capacity>(to_pad: ArrayString<U23>) -> ArrayString<T> {
    pad_with_char::<T>('0', to_pad)
}

#[cfg(test)]
mod pad_tests {
    use super::*;

    // #[test]
    // fn does_not_pad_str_of_size() {
    //     assert_eq!("foo", &*pad_with_char('a', 3, "foo".into()))
    // }

    // #[test]
    // fn single_char_pad() {
    //     assert_eq!("afoo", &*pad_with_char('a', 4, "foo".into()))
    // }

    // #[test]
    // fn multichar_pad() {
    //     assert_eq!("aaafoo", &*pad_with_char('a', 6, "foo".into()))
    // }

    // #[test]
    // fn smaller_pad() {
    //     assert_eq!("c", &*pad_with_char('a', 1, "abc".into()))
    // }

    // #[test]
    // fn pad_0s() {
    //     assert_eq!("00foo", &*pad(5, "foo".into()))
    // }
}

#[cfg(test)]
mod radix_str_tests {
    use super::*;

    // #[test]
    // fn hex_number_below_radix() {
    //     assert_eq!("8", &*to_base36_string(16, 8u8).unwrap());
    // }

    // #[test]
    // fn hex_number_below_radix_letter() {
    //     assert_eq!("a", &*to_base36_string(16, 10u8).unwrap());
    // }

    // #[test]
    // fn number_above_radix() {
    //     assert_eq!("10", &*to_base36_string(16, 16u8).unwrap())
    // }

    // #[test]
    // fn number_well_above_radix() {
    //     assert_eq!("16i", &*to_base36_string(32, 1234u16).unwrap())
    // }

    // #[test]
    // fn large_base_36() {
    //     assert_eq!("7cik2", &*to_base36_string(36, 12341234u32).unwrap())
    // }
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
