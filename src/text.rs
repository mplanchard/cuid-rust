use std::char;
use std::f64;

use crate::BASE;
use crate::error::CuidError;


fn digits_in_base<N: Into<f64>>(base: u8, number: N) -> u64 {
    number.into().log(base as f64) as u64 + 1
}


fn to_radix_string<N: Into<u128>>(radix: u8, number: N) -> Result<String, CuidError> {
    let mut number = number.into();
    if number < radix.into() {
        // No need to allocate a vector or do any math
        // NOTE: we are okay to cast to u32 here, b/c number < radix,
        // which has to be 255 or below.
        return char::from_digit(number as u32, radix.into())
            .map(|c| c.to_string())
            .ok_or(CuidError::TextError("Bad digit"))
    }
    else if number > f64::MAX as u128 {
        return Err(CuidError::TextError("Input number too large"));
    }

    let mut chars: Vec<char> = Vec::with_capacity(
        digits_in_base(radix, number as f64) as usize
    );
    while number > 0 {
        chars.push(
            char::from_digit((number % radix as u128) as u32, radix.into()).unwrap()
        );
        number = number / radix as u128;
    }
    Ok(chars.iter().rev().collect::<String>())
}


pub fn to_base_string<N: Into<u128>>(number: N) -> Result<String, CuidError> {
    to_radix_string(BASE, number)
}


fn pad_with_char<S: AsRef<str>>(pad_char: char, size: usize, to_pad: S) -> String {
    let pad_ref = to_pad.as_ref();
    let length = pad_ref.len();
    if length == size {
        return pad_ref.into();
    }
    else if length > size {
        return pad_ref[length - size..].into();
    }
    let mut ret = String::with_capacity(size as usize);
    for _ in 0..(size - length) {
        ret.push(pad_char);
    }
    ret.push_str(pad_ref);
    ret
}


pub fn pad<S: AsRef<str>>(size: usize, to_pad: S) -> String {
    pad_with_char('0', size, to_pad.as_ref())
}


#[cfg(test)]
mod pad_tests {
    use super::*;

    #[test]
    fn does_not_pad_str_of_size() {
        assert_eq!("foo", &*pad_with_char('a', 3, "foo"))
    }

    #[test]
    fn single_char_pad() {
        assert_eq!("afoo", &*pad_with_char('a', 4, "foo"))
    }

    #[test]
    fn multichar_pad() {
        assert_eq!("aaafoo", &*pad_with_char('a', 6, "foo"))
    }

    #[test]
    fn smaller_pad() {
        assert_eq!("c", &*pad_with_char('a', 1, "abc"))
    }

    #[test]
    fn pad_0s() {
        assert_eq!("00foo", &*pad(5, "foo"))
    }

}


#[cfg(test)]
mod radix_str_tests {
    use super::*;

    #[test]
    fn digits_in_base_7() {
        assert_eq!(4, digits_in_base(7, 1446))
    }

    #[test]
    fn digits_in_base_4() {
        assert_eq!(3, digits_in_base(4, 48))
    }

    #[test]
    fn hex_number_below_radix() {
        assert_eq!("8", &*to_radix_string(16, 8u8).unwrap());
    }

    #[test]
    fn hex_number_below_radix_letter() {
        assert_eq!("a", &*to_radix_string(16, 10u8).unwrap());
    }

    #[test]
    fn number_above_radix() {
        assert_eq!("10", &*to_radix_string(16, 16u8).unwrap())
    }

    #[test]
    fn number_well_above_radix() {
        assert_eq!("16i", &*to_radix_string(32, 1234u16).unwrap())
    }

    #[test]
    fn large_base_36() {
        assert_eq!("7cik2", &*to_radix_string(36, 12341234u32).unwrap())
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
