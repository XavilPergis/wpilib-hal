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

#[inline(always)]
pub fn initialize(mode: CounterMode) -> HalResult<(CounterHandle, i32)> {
    let mut index = 0;
    let handle = unsafe { hal_call!(HAL_InitializeCounter(mode, &mut index))? };
    Ok((handle, index))
}

#[inline(always)]
pub fn free(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_FreeCounter(handle)) }
}

#[inline(always)]
pub fn set_average_size(handle: CounterHandle, size: i32) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterAverageSize(handle, size)) }
}

#[inline(always)]
pub fn set_up_source(handle: CounterHandle, digital_source_handle: DigitalHandle, trigger_type: AnalogTriggerType) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterUpSource(handle, digital_source_handle, trigger_type)) }
}

#[inline(always)]
pub fn set_up_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterUpSourceEdge(handle, rising_edge as NativeBool, falling_edge as NativeBool)) }
}

#[inline(always)]
pub fn clear_up_source(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_ClearCounterUpSource(handle)) }
}

#[inline(always)]
pub fn set_down_source(handle: CounterHandle, digital_source_handle: DigitalHandle, analog_trigger_type: AnalogTriggerType) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterDownSource(handle, digital_source_handle, analog_trigger_type)) }
}

#[inline(always)]
pub fn set_down_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterDownSourceEdge(handle, rising_edge as NativeBool, falling_edge as NativeBool)) }
}

#[inline(always)]
pub fn clear_down_source(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_ClearCounterDownSource(handle)) }
}

#[inline(always)]
pub fn set_up_down_mode(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterUpDownMode(handle)) }
}

#[inline(always)]
pub fn set_external_direction_mode(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterExternalDirectionMode(handle)) }
}

#[inline(always)]
pub fn set_semi_period_mode(handle: CounterHandle, high_semi_period: bool) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterSemiPeriodMode(handle, high_semi_period as NativeBool)) }
}

#[inline(always)]
pub fn set_pulse_length_mode(handle: CounterHandle, threshold: f64) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterPulseLengthMode(handle, threshold)) }
}

#[inline(always)]
pub fn get_samples_to_average(handle: CounterHandle) -> HalResult<i32> {
    unsafe { hal_call!(HAL_GetCounterSamplesToAverage(handle)) }
}

#[inline(always)]
pub fn set_samples_to_average(handle: CounterHandle, samples_to_average: i32) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterSamplesToAverage(handle, samples_to_average)) }
}

#[inline(always)]
pub fn reset(handle: CounterHandle) -> HalResult<()> {
    unsafe { hal_call!(HAL_ResetCounter(handle)) }
}

#[inline(always)]
pub fn get(handle: CounterHandle) -> HalResult<i32> {
    unsafe { hal_call!(HAL_GetCounter(handle)) }
}

#[inline(always)]
pub fn get_period(handle: CounterHandle) -> HalResult<f64> {
    unsafe { hal_call!(HAL_GetCounterPeriod(handle)) }
}

#[inline(always)]
pub fn set_max_period(handle: CounterHandle, max_period: f64) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterMaxPeriod(handle, max_period)) }
}

#[inline(always)]
pub fn set_update_when_empty(handle: CounterHandle, enabled: bool) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterUpdateWhenEmpty(handle, enabled as NativeBool)) }
}

#[inline(always)]
pub fn get_stopped(handle: CounterHandle) -> HalResult<bool> {
    unsafe { hal_call!(HAL_GetCounterStopped(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_direction(handle: CounterHandle) -> HalResult<bool> {
    unsafe { hal_call!(HAL_GetCounterDirection(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn set_reverse_direction(handle: CounterHandle, reverse_direction: bool) -> HalResult<()> {
    unsafe { hal_call!(HAL_SetCounterReverseDirection(handle, reverse_direction as NativeBool)) }
}
