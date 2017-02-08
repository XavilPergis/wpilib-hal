use raw::*;
use std::ffi::NulError;
use std::error::Error;
use std::fmt;

/// Result type encompassing most errors that are returned in this library
pub type HalResult<T> = Result<T, HalError>;

// Because it turns out WPILIB is really messy and have two ways of indicating failure...
// Or not at all...

/// Call a HAL function and wrap the output in a `HalResult`
#[macro_export]
macro_rules! hal_call {
    (ptr $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($($arg,)* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err($crate::error::HalError::from(status)) }
    }};
    (ret $function:ident($($arg:expr),*)) => {{
        let status = unsafe { $function($($arg,)*) };
        if status == 0 { Ok(()) } else { Err($crate::error::HalError::from(status)) }
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

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            FfiError::SampleRateTooHigh => "Sample rate too high".into(),
            FfiError::VoltageOutOfRange => "Voltage out of range".into(),
            FfiError::LoopTimingError => "Loop timing error".into(),
            FfiError::SpiWriteNoMosi => "SPI write no MOSI (Master Out, Slave In)".into(),
            FfiError::SpiReadNoMiso => "SPI read no MISO (Master In, Slave Out)".into(),
            FfiError::SpiReadNoData => "SPI read no data".into(),
            FfiError::IncompatibleState => "Incompatible state".into(),
            FfiError::NoAvailableResources => "No available resources".into(),
            FfiError::NullParameter => "Null parameter".into(),
            FfiError::AnalogTriggerLimitOrderError => "Analog trigger limit order".into(), // TODO
            FfiError::AnalogTriggerPuseOutputError => "Analog trigger pulse output".into(), // TODO
            FfiError::ParameterOutOfRange => "Parameter out of range".into(),
            FfiError::ResourceIsAllocated => "Resource is already allocated".into(),
            FfiError::ResourceOutOfRange => "Resource is out of range".into(),
            FfiError::InvalidAccumulatorChannel => "Channel is not an accumulator channel".into(),
            FfiError::CounterNotSupported => "Counter is not supported".into(),
            FfiError::PwmScaleError => "PWM scale error".into(),
            FfiError::HandleError => "Handle error".into(),
            FfiError::SerialPortNotFound => "Serial port not found".into(),
            FfiError::SerialPortNotOpen => "Serial port not open".into(),
            FfiError::SerialPortError => "Serial port error".into(),
            FfiError::ThreadPriorityError => "Thread priority error".into(),
            FfiError::ThreadPriorityRangeError => "Thread priorite range error".into(),

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

/// An aggregate type that represents all types of errors that could be returned by this crate
#[derive(Debug)]
pub enum HalError {
    /// An FFI error
    Hal(FfiError),
    /// A string that was provided contained a null byte and could not be converted into a CString
    NullError(NulError),
    /// Tried to create a resource struct, but its handle was already initialized
    ResourceAlreadyInitialized,
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            HalError::Hal(ref ffi_err) => ffi_err.description(),
            HalError::NullError(ref nul_err) => nul_err.description(),
            HalError::ResourceAlreadyInitialized => {
                "Tried to create a resource that was already initialized"
            }
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
