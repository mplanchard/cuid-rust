use std::char;
use std::iter;

use super::BASE;


fn to_radix_str<N: Into<u64>>(radix: u8, number: N) -> String {
    let number = number.into();
    let radix = radix as u64;
    if number < radix {
        return char::from_digit(number as u32, radix as u32)
            .unwrap()
            .to_string();
    }

    let mut remainders: Vec<u8> = Vec::new();
    let mut num = number;

    while num > 0 {
        remainders.push((num % radix) as u8);
        num = num / radix;
    }

    remainders
        .into_iter()
        .rev()
        .map(|r| char::from_digit(r as u32, radix as u32).unwrap())
        .collect::<String>()

}

pub fn to_base_str<N: Into<u64>>(number: N) -> String {
    to_radix_str(BASE, number.into())
}


fn pad_with_char<S: Into<String>>(pad_char: char, size: u32, to_pad: S) -> String {
    let to_pad = to_pad.into();
    let size = size as usize;
    let length = to_pad.len();
    iter::repeat(pad_char)
        .take(if size > length { size - length } else { 0 })
        .chain(to_pad.chars())
        .skip(if length > size { length - size } else { 0 })
        .collect()
}


pub fn pad<S: Into<String>>(size: u32, to_pad: S) -> String {
    pad_with_char('0', size, to_pad)
}


#[cfg(test)]
mod pad_tests {
    use super::*;

    #[test]
    fn does_not_pad_str_of_size() {
        assert_eq!("foo", pad_with_char('a', 3, "foo"))
    }

    #[test]
    fn single_char_pad() {
        assert_eq!("afoo", pad_with_char('a', 4, "foo"))
    }

    #[test]
    fn multichar_pad() {
        assert_eq!("aaafoo", pad_with_char('a', 6, "foo"))
    }

    #[test]
    fn smaller_pad() {
        assert_eq!("c", pad_with_char('a', 1, "abc"))
    }

    #[test]
    fn pad_0s() {
        assert_eq!("00foo", pad(5, "foo"))
    }
}


#[cfg(test)]
mod radix_str_tests {
    use super::*;

    #[test]
    fn hex_number_below_radix() {
        assert_eq!("8", to_radix_str(16, 8u8));
    }

    #[test]
    fn hex_number_below_radix_letter() {
        assert_eq!("a", to_radix_str(16, 10u8));
    }

    #[test]
    fn number_above_radix() {
        assert_eq!("10", to_radix_str(16, 16u8))
    }

    #[test]
    fn number_well_above_radix() {
        assert_eq!("16i", to_radix_str(32, 1234u16))
    }

    #[test]
    fn large_base_36() {
        assert_eq!("7cik2", to_radix_str(36, 12341234u32))
    }

}
