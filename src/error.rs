use std::error::Error;
use std::fmt;

/// Result type encompassing most errors that are returned in this library
pub type HalResult<T> = Result<T, HalError>;

/// Call a HAL function and wrap the output in a `HalResult`
#[macro_export]
macro_rules! hal_call {
    // Most HAL functions that have a status code are like this
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = $function($($arg,)* &mut status as *mut i32);
        if status == 0 { Ok(result) } else { Err($crate::error::HalError::Hal($crate::error::FfiError(status))) }
    }};

    (ret $function:ident($($arg:expr),*)) => {{
        let status = $function($($arg,)*);
        if status == 0 { Ok(()) } else { Err($crate::error::HalError::Hal($crate::error::FfiError(status))) }
    }};
}

/// An error as emitted by WPILib
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FfiError(pub(crate) i32);

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::hal::get_error_message(self.0))
    }
}

impl Error for FfiError {
    fn description(&self) -> &str {
        "FFI returned bad status code"
    }
}

/// An aggregate type that represents all types of errors that could be
/// returned by this crate
#[derive(Debug)]
pub enum HalError {
    /// An FFI error
    Hal(FfiError),
    /// Tried to use a channel that could not be used for whatever type of device this was returned from
    InvalidChannel(i32),
    InvalidModule(i32),
    /// Some other custom error
    Other(Box<Error + Send + Sync>),
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            HalError::Hal(ref ffi_err) => ffi_err.description(),
            HalError::InvalidChannel(_) => "Invalid channel",
            HalError::InvalidModule(_) => "Invalid module",
            HalError::Other(ref err) => err.description(),
        };

        write!(f, "{}", msg)
    }
}

impl Error for HalError {
    fn description(&self) -> &str {
        "Error communicating with the HAL"
    }
}

impl From<Box<Error + Send + Sync>> for HalError {
    fn from(err: Box<Error + Send + Sync>) -> HalError {
        HalError::Other(err)
    }
}
