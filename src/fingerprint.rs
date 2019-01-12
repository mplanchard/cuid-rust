use std::process;

use hostname::get_hostname;

use text::{ to_base_str, pad };

use super::BASE;


static FINGERPRINT_PADDING: u8 = 2;


fn pid() -> String {
    pad(FINGERPRINT_PADDING as u32, &to_base_str(process::id()))
}


fn convert_hostname(hostname_getter: fn() -> Option<String>) -> String {
    pad(
        FINGERPRINT_PADDING as u32,
        &to_base_str(
            hostname_getter()
                .map(|h| {
                    h.chars().fold(
                        h.len() + BASE as usize,
                        |acc, c| acc + c as usize
                    )
                }).unwrap() as u64
        )
    )
}


#[cfg(test)]
mod fingerprint_tests {

    use super::*;

    #[test]
    fn test_pid_length() {
        assert_eq!(pid().len(), FINGERPRINT_PADDING as usize)
    }

    #[test]
    fn test_convert_hostname_1() {
        assert_eq!("a3", convert_hostname(|| Some("foo".into())))
    }

    #[test]
    fn test_convert_hostname_2() {
        assert_eq!("9o", convert_hostname(|| Some("bar".into())))
    }

    #[test]
    fn test_convert_hostname_3() {
        assert_eq!("nf", convert_hostname(|| Some("mr-magoo".into())))
    }

    #[test]
    fn test_convert_hostname_4() {
        assert_eq!(
            "j9",
            convert_hostname(|| Some("wow-what-a-long-hostname-you-have".into()))
        )
    }

}
