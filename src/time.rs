
use std::time::{SystemTime, UNIX_EPOCH};

use error::CuidError;
use text::to_base_str;


pub fn timestamp() -> Result<Box<str>, CuidError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|time| time.as_secs())
        .map(to_base_str)
        .map_err(|time_err| CuidError::from(time_err))
}


#[cfg(tests)]
mod time_tests {
    use super::*;
    use super::super::BASE;

    #[test]
    fn test_timestamp() {
        assert!(
            (
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                - u64::from_str_radix(timestamp(), BASE)
            ) < 5
        )
    }
}
