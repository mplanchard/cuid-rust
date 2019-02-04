use std::error as stderr;
use std::fmt;
use std::time::SystemTimeError;


#[derive(Debug)]
pub enum CuidError {
    CounterError,
    FingerprintError,
    TimestampError(SystemTimeError),
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
            CuidError::TimestampError(ref err) => write!(
                f, "SystemTimeError: {}", err
            ),
        }
    }
}

impl stderr::Error for CuidError {
    fn description(&self) -> &str {
        match *self {
            CuidError::CounterError => "Could not retrieve counter",
            CuidError::FingerprintError => "Could not generate fingerprint",
            CuidError::TimestampError(_) => "Could not generate timestamp",
        }
    }
}

impl From<SystemTimeError> for CuidError {
    fn from(err: SystemTimeError) -> Self {
        CuidError::TimestampError(err)
    }
}
