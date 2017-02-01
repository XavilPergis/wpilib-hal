use raw;
use std::ffi::NulError;

/// Result type encompassing most errors that are returned in this library
pub type HalResult<T> = Result<T, HalError>;

// Because it turns out WPILIB is really messy and have two ways of indicating failure...
// Or not at all...
macro_rules! hal_status_pointer_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($($arg,)* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
}

macro_rules! hal_status_return_call {
    ($function:ident($($arg:expr),*)) => {{
        let status = unsafe { $function($($arg,)*) };
        if status == 0 { Ok(()) } else { Err(HalError::from(status)) }
    }};
}

// FIXME: Is the i32 that's returned actually a status code?

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
    Unknown(i32)
}

pub enum HalError {
    Hal(FfiError),
    NullError(NulError),
    IndexOutOfRange
}

impl From<i32> for HalError {
    fn from(code: i32) -> HalError {
        use self::HalError::Hal;
        use self::FfiError::*;

        // Yes, yes, it's messy. But ***HALF OF THE CONSTANTS ARE DIFFERENT TYPES***
        if code >= 0 {
            match code as u32 {
                raw::SAMPLE_RATE_TOO_HIGH              => Hal(SampleRateTooHigh),
                raw::VOLTAGE_OUT_OF_RANGE              => Hal(VoltageOutOfRange),
                raw::LOOP_TIMING_ERROR                 => Hal(LoopTimingError),
                raw::SPI_WRITE_NO_MOSI                 => Hal(SpiWriteNoMosi),
                raw::SPI_READ_NO_MISO                  => Hal(SpiReadNoMiso),
                raw::SPI_READ_NO_DATA                  => Hal(SpiReadNoData),
                raw::INCOMPATIBLE_STATE                => Hal(IncompatibleState),

                k => Hal(Unknown(k as i32))
            }
        } else {
            match code {
                raw::NO_AVAILABLE_RESOURCES            => Hal(NoAvailableResources),
                raw::NULL_PARAMETER                    => Hal(NullParameter),
                raw::ANALOG_TRIGGER_LIMIT_ORDER_ERROR  => Hal(AnalogTriggerLimitOrderError),
                raw::ANALOG_TRIGGER_PULSE_OUTPUT_ERROR => Hal(AnalogTriggerPuseOutputError),
                raw::PARAMETER_OUT_OF_RANGE            => Hal(ParameterOutOfRange),
                raw::RESOURCE_IS_ALLOCATED             => Hal(ResourceIsAllocated),
                raw::RESOURCE_OUT_OF_RANGE             => Hal(ResourceOutOfRange),
                raw::HAL_INVALID_ACCUMULATOR_CHANNEL   => Hal(InvalidAccumulatorChannel),
                raw::HAL_COUNTER_NOT_SUPPORTED         => Hal(CounterNotSupported),
                raw::HAL_PWM_SCALE_ERROR               => Hal(PwmScaleError),
                raw::HAL_HANDLE_ERROR                  => Hal(HandleError),
                raw::HAL_SERIAL_PORT_NOT_FOUND         => Hal(SerialPortNotFound),
                raw::HAL_SERIAL_PORT_OPEN_ERROR        => Hal(SerialPortNotOpen),
                raw::HAL_SERIAL_PORT_ERROR             => Hal(SerialPortError),
                raw::HAL_THREAD_PRIORITY_ERROR         => Hal(ThreadPriorityError),
                raw::HAL_THREAD_PRIORITY_RANGE_ERROR   => Hal(ThreadPriorityRangeError),

                k => Hal(Unknown(k))
            }
        }
    }
}

impl From<NulError> for HalError {
    fn from(err: NulError) -> HalError { HalError::NullError(err) }
}
