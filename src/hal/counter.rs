use ::raw::*;

// FIXME
pub fn initialize_counter(mode: CounterMode, index: *mut i32) -> HalResult<HAL_CounterHandle> {
    hal_call![ ptr HAL_InitializeCounter(mode.into()) ]
}

pub fn free_counter(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreeCounter(handle.get_handle()) ]
}

pub fn set_counter_average_size(handle: CounterHandle, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterAverageSize(handle.get_handle(), size) ]
}

pub fn set_counter_up_source<H>(handle: CounterHandle, digital_source_handle: &H, trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
    hal_call![ ptr HAL_SetCounterUpSource(handle.get_handle(), digital_source_handle.get_handle(), trigger_type.into()) ]
}

pub fn set_counter_up_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}

pub fn clear_counter_up_source(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ClearCounterUpSource(handle.get_handle()) ]
}

pub fn set_counter_down_source<H>(handle: CounterHandle, digital_source_handle: &H, analog_trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
    hal_call![ ptr HAL_SetCounterDownSource(handle.get_handle(), digital_source_handle.get_handle(), analog_trigger_type.into()) ]
}

pub fn set_counter_down_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterDownSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}

pub fn clear_counter_down_source(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ClearCounterDownSource(handle.get_handle()) ]
}

pub fn set_counter_up_down_mode(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpDownMode(handle.get_handle()) ]
}

pub fn set_counter_external_direction_mode(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterExternalDirectionMode(handle.get_handle()) ]
}

pub fn set_counter_semi_period_mode(handle: CounterHandle, high_semi_period: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterSemiPeriodMode(handle.get_handle(), high_semi_period as HAL_Bool) ]
}

pub fn set_counter_pulse_length_mode(handle: CounterHandle, threshold: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterPulseLengthMode(handle.get_handle()) ]
}

pub fn get_counter_samples_to_average(handle: CounterHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetCounterSamplesToAverage(handle.get_handle()) ]
}

pub fn set_counter_samples_to_average(handle: CounterHandle, samples_to_average: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterSamplesToAverage(handle.get_handle(), samples_to_average) ]
}

pub fn reset_counter(handle: CounterHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetCounter(handle.get_handle()) ]
}

pub fn get_counter(handle: CounterHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetCounter(handle.get_handle()) ]
}

pub fn get_counter_period(handle: CounterHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetCounterPeriod(handle.get_handle()) ]
}

pub fn set_counter_max_period(handle: CounterHandle, max_period: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterMaxPeriod(handle.get_handle(), max_period) ]
}

pub fn set_counter_update_when_empty(handle: CounterHandle, enabled: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterUpdateWhenEmpty(handle.get_handle(), enabled as HAL_Bool) ]
}

pub fn get_counter_stopped(handle: CounterHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCounterStopped(handle.get_handle()) != 0 ]
}

pub fn get_counter_direction(handle: CounterHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCounterDirection(handle.get_handle()) != 0 ]
}

pub fn set_counter_reverse_direction(handle: CounterHandle, reverse_direction: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCounterReverseDirection(handle.get_handle(), reverse_direction as HAL_Bool) ]
}
