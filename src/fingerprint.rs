use std::process;

use hostname::get_hostname;

use text::{ to_base_str, pad };


static FINGERPRINT_PADDING: u8 = 2;


fn pid() -> String {
    pad(FINGERPRINT_PADDING as u32, to_base_str(process::id()))
}


#[cfg(test)]
mod fingerprint_tests {

    use super::*;

    #[test]
    fn test_pid_length() {
        assert_eq!(pid().len(), FINGERPRINT_PADDING as usize)
    }

}
