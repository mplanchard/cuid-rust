use std::error as stderr;
use std::fmt;
use std::time::SystemTimeError;


#[derive(Debug)]
pub enum CuidError {
    CounterError,
    FingerprintError(&'static str),
    TimestampError(SystemTimeError),
    TextError(&'static str),
}

impl fmt::Display for CuidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CuidError::CounterError => write!(
                f, "Could not retrieve counter value!"
            ),
            CuidError::FingerprintError(ref err) => write!(
                f, "Could not generate fingerprint: {}", err
            ),
            CuidError::TimestampError(ref err) => write!(
                f, "SystemTimeError: {}", err
            ),
            CuidError::TextError(ref err) => write!(
                f, "TextError: {}", err
            )
        }
    }
}

impl stderr::Error for CuidError {
    fn description(&self) -> &str {
        match *self {
            CuidError::CounterError => "Could not retrieve counter",
            CuidError::FingerprintError(_) => "Could not generate fingerprint",
            CuidError::TimestampError(_) => "Could not generate timestamp",
            CuidError::TextError(_) => "Error processing text",
        }
    }
}

impl From<SystemTimeError> for CuidError {
    fn from(err: SystemTimeError) -> Self {
        CuidError::TimestampError(err)
    }
}
