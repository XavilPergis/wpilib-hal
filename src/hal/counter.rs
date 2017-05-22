use ::error::*;
use hal::analog_trigger::AnalogTriggerType;
use hal::handle::*;
use ::raw::*;

pub type RawCounterMode = HAL_Counter_Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CounterMode {
    TwoPulse,
    Semiperiod,
    PulseLength,
    ExternalDirection,
}

impl_convert! {
    HAL_Counter_Mode, CounterMode;
    HAL_Counter_kExternalDirection <=> ExternalDirection,
    HAL_Counter_kPulseLength <=> PulseLength,
    HAL_Counter_kSemiperiod <=> Semiperiod,
    HAL_Counter_kTwoPulse <=> TwoPulse
}

// FIXME
pub unsafe fn initialize(mode: CounterMode, index: &mut i32) -> HalResult<CounterHandle> {
    hal_call![ ptr HAL_InitializeCounter(mode.into(), index) ]
}

pub unsafe fn free(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreeCounter(handle) ]
}

pub unsafe fn set_average_size(handle: CounterHandle, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterAverageSize(handle, size) ]
}

pub unsafe fn set_up_source(handle: CounterHandle, digital_source_handle: DigitalHandle, trigger_type: AnalogTriggerType) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpSource(handle, digital_source_handle, trigger_type.into()) ]
}

pub unsafe fn set_up_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpSourceEdge(handle, rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}

pub unsafe fn clear_up_source(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ClearCounterUpSource(handle) ]
}

pub unsafe fn set_down_source(handle: CounterHandle, digital_source_handle: DigitalHandle, analog_trigger_type: AnalogTriggerType) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterDownSource(handle, digital_source_handle, analog_trigger_type.into()) ]
}

pub unsafe fn set_down_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterDownSourceEdge(handle, rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}

pub unsafe fn clear_down_source(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ClearCounterDownSource(handle) ]
}

pub unsafe fn set_up_down_mode(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpDownMode(handle) ]
}

pub unsafe fn set_external_direction_mode(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterExternalDirectionMode(handle) ]
}

pub unsafe fn set_semi_period_mode(handle: CounterHandle, high_semi_period: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterSemiPeriodMode(handle, high_semi_period as HAL_Bool) ]
}

pub unsafe fn set_pulse_length_mode(handle: CounterHandle, threshold: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterPulseLengthMode(handle, threshold) ]
}

pub unsafe fn get_samples_to_average(handle: CounterHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetCounterSamplesToAverage(handle) ]
}

pub unsafe fn set_samples_to_average(handle: CounterHandle, samples_to_average: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterSamplesToAverage(handle, samples_to_average) ]
}

pub unsafe fn reset(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetCounter(handle) ]
}

pub unsafe fn get(handle: CounterHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetCounter(handle) ]
}

pub unsafe fn get_period(handle: CounterHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetCounterPeriod(handle) ]
}

pub unsafe fn set_max_period(handle: CounterHandle, max_period: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterMaxPeriod(handle, max_period) ]
}

pub unsafe fn set_update_when_empty(handle: CounterHandle, enabled: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpdateWhenEmpty(handle, enabled as HAL_Bool) ]
}

pub unsafe fn get_stopped(handle: CounterHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCounterStopped(handle) ].map(|n| n != 0)
}

pub unsafe fn get_direction(handle: CounterHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCounterDirection(handle) ].map(|n| n != 0)
}

pub unsafe fn set_reverse_direction(handle: CounterHandle, reverse_direction: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterReverseDirection(handle, reverse_direction as HAL_Bool) ]
}
