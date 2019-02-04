use std::error as stderr;
use std::fmt;


#[derive(Debug)]
pub enum CuidError {
    CounterError,
    FingerprintError,
    TimestampError,
}

impl fmt::Display for CuidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CuidError::CounterError => write!(
                f, "Could not retrieve counter value!"
            ),
            CuidError::FingerprintError => write!(
                f, "Could not create system fingerprint!"
            ),
            CuidError::TimestampError => write!(
                f, "Could not calculate time since Epoch!"
            ),
        }
    }
}

impl stderr::Error for CuidError {
    fn description(&self) -> &str {
        match *self {
            CuidError::CounterError => "Could not retrieve counter",
            CuidError::FingerprintError => "Could not generate fingerprint",
            CuidError::TimestampError => "Could not generate timestamp",
        }
    }
}
