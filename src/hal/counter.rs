use error::*;
use hal::analog_trigger::AnalogTriggerType;
use hal::types::{CounterHandle, Handle, DigitalHandle, NativeBool};

extern "C" {
    fn HAL_InitializeCounter(mode: CounterMode, index: *mut i32, status: *mut i32) -> CounterHandle;
    fn HAL_FreeCounter(handle: CounterHandle, status: *mut i32);
    fn HAL_SetCounterAverageSize(handle: CounterHandle, size: i32, status: *mut i32);
    fn HAL_SetCounterUpSource(handle: CounterHandle, digitalSourceHandle: Handle, analogTriggerType: AnalogTriggerType, status: *mut i32);
    fn HAL_SetCounterUpSourceEdge(handle: CounterHandle, risingEdge: NativeBool, fallingEdge: NativeBool, status: *mut i32);
    fn HAL_ClearCounterUpSource(handle: CounterHandle, status: *mut i32);
    fn HAL_SetCounterDownSource(handle: CounterHandle, digitalSourceHandle: Handle, analogTriggerType: AnalogTriggerType, status: *mut i32);
    fn HAL_SetCounterDownSourceEdge(handle: CounterHandle, risingEdge: NativeBool, fallingEdge: NativeBool, status: *mut i32);
    fn HAL_ClearCounterDownSource(handle: CounterHandle, status: *mut i32);
    fn HAL_SetCounterUpDownMode(handle: CounterHandle, status: *mut i32);
    fn HAL_SetCounterExternalDirectionMode(handle: CounterHandle, status: *mut i32);
    fn HAL_SetCounterSemiPeriodMode(handle: CounterHandle, highSemiPeriod: NativeBool, status: *mut i32);
    fn HAL_SetCounterPulseLengthMode(handle: CounterHandle, threshold: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_GetCounterSamplesToAverage(handle: CounterHandle, status: *mut i32) -> i32;
    fn HAL_SetCounterSamplesToAverage(handle: CounterHandle, samplesToAverage: i32, status: *mut i32);
    fn HAL_ResetCounter(handle: CounterHandle, status: *mut i32);
    fn HAL_GetCounter(handle: CounterHandle, status: *mut i32) -> i32;
    fn HAL_GetCounterPeriod(handle: CounterHandle, status: *mut i32) -> ::std::os::raw::c_double;
    fn HAL_SetCounterMaxPeriod(handle: CounterHandle, maxPeriod: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_SetCounterUpdateWhenEmpty(handle: CounterHandle, enabled: NativeBool, status: *mut i32);
    fn HAL_GetCounterStopped(handle: CounterHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCounterDirection(handle: CounterHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetCounterReverseDirection(handle: CounterHandle, reverseDirection: NativeBool, status: *mut i32);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CounterMode {
    TwoPulse = 0,
    Semiperiod = 1,
    PulseLength = 2,
    ExternalDirection = 3,
}
