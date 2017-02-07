use ::raw::*;

//pub fn initialize_analog_trigger(handle: AnalogInputHandle)
// FIXME
//    pub fn HAL_InitializeAnalogTrigger(portHandle: HAL_AnalogInputHandle,
//                                       index: *mut i32,
//                                       status: *mut i32)
//                                       -> HAL_AnalogTriggerHandle;

pub fn clean_analog_trigger(handle: AnalogTriggerHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanAnalogTrigger(handle.get_handle()) ]
}

pub fn set_analog_trigger_limits_raw(handle: AnalogTriggerHandle, lower: i32, upper: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerLimitsRaw(handle.get_handle(), lower, upper) ]
}

pub fn set_analog_trigger_limits_voltage(handle: AnalogTriggerHandle, lower: f64, upper: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerLimitsVoltage(handle.get_handle(), lower, upper) ]
}

pub fn set_analog_trigger_avergaed(handle: AnalogTriggerHandle, use_averaged_value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerAveraged(handle.get_handle(), use_averaged_value as HAL_Bool) ]
}

pub fn set_analog_trigger_filtered(handle: AnalogTriggerHandle, use_filtered_value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerFiltered(handle.get_handle(), use_filtered_value as HAL_Bool) ]
}

pub fn get_analog_trigger_in_window(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerInWindow(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_analog_trigger_state(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerTriggerState(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_analog_trigger_output(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerOutput(handle.get_handle(), trigger_type.into()) ].map(|n| n != 0)
}
