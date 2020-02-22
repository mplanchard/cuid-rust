use std::error as stderr;
use std::ffi;
use std::io;
use std::fmt;
use std::num;
use std::time::SystemTimeError;


/// Errors for the CUID library
#[derive(Debug)]
pub enum CuidError {
    CounterError,
    IntegerConversionError(num::TryFromIntError),
    FingerprintError(&'static str),
    IOError(io::Error),
    OsStringError(ffi::OsString),
    TextError(&'static str),
    TimestampError(SystemTimeError),
}

impl fmt::Display for CuidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CuidError::CounterError => write!(
                f, "Could not retrieve counter value!"
            ),
            CuidError::IntegerConversionError(err) => write!(
                f, "Failed to convert integer: {}", err
            ),
            CuidError::FingerprintError(err) => write!(
                f, "Could not generate fingerprint: {}", err
            ),
            CuidError::IOError(err) => write!(
                f, "Error reading or writing to the system: {}", err
            ),
            CuidError::OsStringError(err) => write!(
                f, "Failed to convert Operating System String: {:?}", err
            ),
            CuidError::TextError(err) => write!(
                f, "TextError: {}", err
            ),
            CuidError::TimestampError(err) => write!(
                f, "SystemTimeError: {}", err
            ),
        }
    }
}

impl stderr::Error for CuidError {
    fn description(&self) -> &str {
        match self {
            CuidError::CounterError => "Could not retrieve counter",
            CuidError::IntegerConversionError(_) => "Failed to convert integer",
            CuidError::FingerprintError(_) => "Could not generate fingerprint",
            CuidError::IOError(_) => "Failed performing system IO",
            CuidError::OsStringError(_) => "Could not convert OsString",
            CuidError::TextError(_) => "Error processing text",
            CuidError::TimestampError(_) => "Could not generate timestamp",
        }
    }
}

impl From<ffi::OsString> for CuidError {
    fn from(err: ffi::OsString) -> Self {
        CuidError::OsStringError(err)
    }
}
impl From<num::TryFromIntError> for CuidError {
    fn from(err: num::TryFromIntError) -> Self {
        CuidError::IntegerConversionError(err)
    }
}
impl From<SystemTimeError> for CuidError {
    fn from(err: SystemTimeError) -> Self {
        CuidError::TimestampError(err)
    }
}
impl From<io::Error> for CuidError {
    fn from(err: io::Error) -> Self {
        CuidError::IOError(err)
    }
}
