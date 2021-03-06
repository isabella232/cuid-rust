use std::process;

use error::CuidError;
use hostname::get_hostname;

use text::{pad, to_base_str};

use super::BASE;

static FINGERPRINT_PADDING: u8 = 2;

fn pid() -> Result<Box<str>, CuidError> {
    to_base_str(process::id())
        .map(|s| pad(FINGERPRINT_PADDING as u32, &s))
        .map_err(|_| CuidError::FingerprintError("Could not encode pid"))
}

fn convert_hostname(hostname_getter: fn() -> Option<String>) -> Result<Box<str>, CuidError> {
    hostname_getter()
        .map(|h| {
            h.chars()
                .fold(h.len() + BASE as usize, |acc, c| acc + c as usize)
        })
        .map(|print| print as u64)
        .map(to_base_str)
        .unwrap_or(Err(CuidError::FingerprintError(
            "Could not retrieve hostname",
        )))
        .map(|base_str| pad(FINGERPRINT_PADDING as u32, &base_str))
        .map(|box_str| box_str.into())
}

fn host_id() -> Result<Box<str>, CuidError> {
    convert_hostname(get_hostname)
}

pub fn fingerprint() -> Result<Box<str>, CuidError> {
    let hid = host_id()?;
    let procid = pid()?;
    Ok([procid, hid].concat().into())
}

#[cfg(test)]
mod fingerprint_tests {

    use super::*;

    #[test]
    fn test_pid_length() {
        assert_eq!(pid().unwrap().len(), FINGERPRINT_PADDING as usize)
    }

    // The below expected host_ids were all generated directly using
    // the original javascript implementation.

    #[test]
    fn test_convert_hostname_1() {
        assert_eq!("a3", &*convert_hostname(|| Some("foo".into())).unwrap())
    }

    #[test]
    fn test_convert_hostname_2() {
        assert_eq!("9o", &*convert_hostname(|| Some("bar".into())).unwrap())
    }

    #[test]
    fn test_convert_hostname_3() {
        assert_eq!(
            "nf",
            &*convert_hostname(|| Some("mr-magoo".into())).unwrap()
        )
    }

    #[test]
    fn test_convert_hostname_4() {
        assert_eq!(
            "j9",
            &*convert_hostname(|| Some("wow-what-a-long-hostname-you-have".into())).unwrap()
        )
    }

    #[test]
    fn fingerprint_len() {
        assert_eq!(4, fingerprint().unwrap().len())
    }

}
