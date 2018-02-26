use std::error::Error;
use std::ffi::NulError;
use std::fmt;

/// Result type encompassing most errors that are returned in this library
pub type HalResult<T> = Result<T, HalError>;

/// Call a HAL function and wrap the output in a `HalResult`
#[macro_export]
macro_rules! hal_call {
    // Most HAL functions that have a status code are like this
    (ptr $function:ident($($arg:expr),*)) => {{
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
    /// A string that was provided contained a null byte and could not be converted into a CString
    NullError(NulError),
    /// Tried to create a resource struct, but its handle was already initialized
    ResourceAlreadyInitialized,
    /// Module did not have the right device for type
    BadModuleType,
    /// Channel did not have the right device for type
    BadChannelType,
    /// Tried to give the incorrect type of handle to a robot IO function
    WrongIoInterface,
    InvalidChannel,
    OutOfRange,
    /// Some other custom error
    Other(Box<Error + Send + Sync>),
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            HalError::Hal(ref ffi_err) => ffi_err.description(),
            HalError::NullError(ref nul_err) => nul_err.description(),
            HalError::ResourceAlreadyInitialized => "Tried to create a resource that was already initialized",
            HalError::BadModuleType => "Module did not have the right device for type",
            HalError::BadChannelType => "Channel did not have the right device for type",
            HalError::WrongIoInterface => "Tried to give the incorrect type of handle to a robot IO function",
            HalError::InvalidChannel => "Invalid channel",
            HalError::OutOfRange => "A channel used does not exist",
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

impl From<NulError> for HalError {
    fn from(err: NulError) -> HalError {
        HalError::NullError(err)
    }
}

impl From<Box<Error + Send + Sync>> for HalError {
    fn from(err: Box<Error + Send + Sync>) -> HalError {
        HalError::Other(err)
    }
}
