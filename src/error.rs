use raw::*;
use std::error::Error;
use std::ffi::NulError;
use std::fmt;

/// Result type encompassing most errors that are returned in this library
pub type HalResult<T> = Result<T, HalError>;

// Because it turns out WPILIB is really messy and have two ways of indicating
// failure...
// Or not at all...

/// Call a HAL function and wrap the output in a `HalResult`
#[macro_export]
macro_rules! hal_call {
    (ptr $function:ident($($arg:expr),*)) => {{
        if $crate::hal::hal_is_initialized() {
            let mut status = 0;
            let result = $function($($arg,)* &mut status as *mut i32);
            if status == 0 { Ok(result) } else { Err($crate::error::HalError::from(status)) }
        } else {
            Err($crate::error::HalError::HalNotInitialized)
        }
    }};
    (ret $function:ident($($arg:expr),*)) => {{
        if $crate::hal::hal_is_initialized() {
            let status = $function($($arg,)*);
            if status == 0 { Ok(()) } else { Err($crate::error::HalError::from(status)) }
        } else {
            Err($crate::error::HalError::HalNotInitialized)
        }
    }};
}

// FIXME: Is the i32 that's returned actually a status code?

/// An error as emitted by WPILib
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum FfiError {
    SampleRateTooHigh,
    VoltageOutOfRange,
    LoopTimingError,
    SpiWriteNoMosi, // TODO: What?
    SpiReadNoMiso,
    SpiReadNoData,
    IncompatibleState,
    NoAvailableResources,
    NullParameter, // Is this really needed? :p
    AnalogTriggerLimitOrderError,
    AnalogTriggerPuseOutputError,
    ParameterOutOfRange,
    ResourceIsAllocated,
    ResourceOutOfRange,
    InvalidAccumulatorChannel,
    CounterNotSupported,
    PwmScaleError,
    HandleError,
    SerialPortNotFound,
    SerialPortNotOpen,
    SerialPortError,
    ThreadPriorityError,
    ThreadPriorityRangeError,

    /// Some other status code that doesn't have an associated variant
    Unknown(i32),
}

macro_rules! arr_to_str {
    ($val:ident) => {{
        use std::str;
        str::from_utf8(&$crate::raw::$val[0..$val.len()-1]).unwrap().to_string()
    }};
}
impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            FfiError::SampleRateTooHigh => arr_to_str!(SAMPLE_RATE_TOO_HIGH_MESSAGE),
            FfiError::VoltageOutOfRange => arr_to_str!(VOLTAGE_OUT_OF_RANGE_MESSAGE),
            FfiError::LoopTimingError => arr_to_str!(LOOP_TIMING_ERROR_MESSAGE),
            FfiError::SpiWriteNoMosi => arr_to_str!(SPI_WRITE_NO_MOSI_MESSAGE),
            FfiError::SpiReadNoMiso => arr_to_str!(SPI_READ_NO_MISO_MESSAGE),
            FfiError::SpiReadNoData => arr_to_str!(SPI_READ_NO_DATA_MESSAGE),
            FfiError::IncompatibleState => arr_to_str!(INCOMPATIBLE_STATE_MESSAGE),
            FfiError::NoAvailableResources => arr_to_str!(NO_AVAILABLE_RESOURCES_MESSAGE),
            FfiError::NullParameter => arr_to_str!(NULL_PARAMETER_MESSAGE),
            FfiError::AnalogTriggerLimitOrderError => arr_to_str!(ANALOG_TRIGGER_LIMIT_ORDER_ERROR_MESSAGE),
            FfiError::AnalogTriggerPuseOutputError => arr_to_str!(ANALOG_TRIGGER_PULSE_OUTPUT_ERROR_MESSAGE),
            FfiError::ParameterOutOfRange => arr_to_str!(PARAMETER_OUT_OF_RANGE_MESSAGE),
            FfiError::ResourceIsAllocated => arr_to_str!(RESOURCE_IS_ALLOCATED_MESSAGE),
            FfiError::ResourceOutOfRange => arr_to_str!(RESOURCE_OUT_OF_RANGE_MESSAGE),
            FfiError::InvalidAccumulatorChannel => arr_to_str!(HAL_INVALID_ACCUMULATOR_CHANNEL_MESSAGE),
            FfiError::CounterNotSupported => arr_to_str!(HAL_COUNTER_NOT_SUPPORTED_MESSAGE),
            FfiError::PwmScaleError => arr_to_str!(HAL_PWM_SCALE_ERROR_MESSAGE),
            FfiError::HandleError => arr_to_str!(HAL_HANDLE_ERROR_MESSAGE),
            FfiError::SerialPortNotFound => arr_to_str!(HAL_SERIAL_PORT_NOT_FOUND_MESSAGE),
            FfiError::SerialPortNotOpen => arr_to_str!(HAL_SERIAL_PORT_OPEN_ERROR_MESSAGE),
            FfiError::SerialPortError => arr_to_str!(HAL_SERIAL_PORT_ERROR_MESSAGE),
            FfiError::ThreadPriorityError => "?".into(),
            FfiError::ThreadPriorityRangeError => "?".into(),

            /// Some other status code that doesn't have an associated variant
            FfiError::Unknown(e) => format!("Unknown error: {}", e),
        };

        write!(f, "{}", msg)
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
    /// A string that was provided contained a null byte and could not be
    /// converted into a CString
    NullError(NulError),
    /// Tried to create a resource struct, but its handle was already
    /// initialized
    ResourceAlreadyInitialized,
    /// HAL was not initialized, but we tried to invoke a HAL function.
    HalNotInitialized,
    /// Module did not have the right device for type
    BadModuleType,
    /// Channel did not have the right device for type
    BadChannelType,
    /// Tried to give the incorrect type of handle to a robot IO function
    WrongIoInterface,
    /// Some other custom error
    Other(Box<Error + Send + Sync>),
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            HalError::Hal(ref ffi_err) => ffi_err.description(),
            HalError::NullError(ref nul_err) => nul_err.description(),
            HalError::ResourceAlreadyInitialized => "Tried to create a resource that was already initialized",
            HalError::HalNotInitialized => "HAL was not initialized, but a HAL function was invoked",
            HalError::BadModuleType => "Module did not have the right device for type",
            HalError::BadChannelType => "Channel did not have the right device for type",
            HalError::WrongIoInterface => "Tried to give the incorrect type of handle to a robot IO function",
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

impl From<i32> for HalError {
    fn from(code: i32) -> HalError {
        use self::HalError::Hal;
        use self::FfiError::*;

        // Yes, yes, it's messy. But ***HALF OF THE CONSTANTS ARE DIFFERENT TYPES***
        if code >= 0 {
            match code as u32 {
                SAMPLE_RATE_TOO_HIGH => Hal(SampleRateTooHigh),
                VOLTAGE_OUT_OF_RANGE => Hal(VoltageOutOfRange),
                LOOP_TIMING_ERROR => Hal(LoopTimingError),
                SPI_WRITE_NO_MOSI => Hal(SpiWriteNoMosi),
                SPI_READ_NO_MISO => Hal(SpiReadNoMiso),
                SPI_READ_NO_DATA => Hal(SpiReadNoData),
                INCOMPATIBLE_STATE => Hal(IncompatibleState),

                k => Hal(Unknown(k as i32)),
            }
        } else {
            match code {
                NO_AVAILABLE_RESOURCES => Hal(NoAvailableResources),
                NULL_PARAMETER => Hal(NullParameter),
                ANALOG_TRIGGER_LIMIT_ORDER_ERROR => Hal(AnalogTriggerLimitOrderError),
                ANALOG_TRIGGER_PULSE_OUTPUT_ERROR => Hal(AnalogTriggerPuseOutputError),
                PARAMETER_OUT_OF_RANGE => Hal(ParameterOutOfRange),
                RESOURCE_IS_ALLOCATED => Hal(ResourceIsAllocated),
                RESOURCE_OUT_OF_RANGE => Hal(ResourceOutOfRange),
                HAL_INVALID_ACCUMULATOR_CHANNEL => Hal(InvalidAccumulatorChannel),
                HAL_COUNTER_NOT_SUPPORTED => Hal(CounterNotSupported),
                HAL_PWM_SCALE_ERROR => Hal(PwmScaleError),
                HAL_HANDLE_ERROR => Hal(HandleError),
                HAL_SERIAL_PORT_NOT_FOUND => Hal(SerialPortNotFound),
                HAL_SERIAL_PORT_OPEN_ERROR => Hal(SerialPortNotOpen),
                HAL_SERIAL_PORT_ERROR => Hal(SerialPortError),
                HAL_THREAD_PRIORITY_ERROR => Hal(ThreadPriorityError),
                HAL_THREAD_PRIORITY_RANGE_ERROR => Hal(ThreadPriorityRangeError),

                k => Hal(Unknown(k)),
            }
        }
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
