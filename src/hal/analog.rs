use std::ops::Range;
use hal::handle::*;
use ::error::*;
use ::raw::*;

/// The raw output of an accumulator.
pub struct AccumulatorOutput {
    value: i64,
    count: i64
}

pub type RawAnalogTriggerType = HAL_AnalogTriggerType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalogTriggerType {
    /// Makes the trigger return true if the value is in the bounds
    InWindow,
    /// Makes the trigger return the last state if in range, true if higher, or false if lower
    State,
    RisingPulse,
    FallingPulse,
}

impl_convert! {
    HAL_AnalogTriggerType, AnalogTriggerType;
    HAL_Trigger_kInWindow <=> InWindow,
    HAL_Trigger_kState <=> State,
    HAL_Trigger_kRisingPulse <=> RisingPulse,
    HAL_Trigger_kFallingPulse <=> FallingPulse
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalogTriggerLimits {
    Raw(Range<i32>),
    Voltage(Range<f64>)
}

pub fn initialize_analog_input_port(handle: PortHandle) -> HalResult<AnalogInputHandle> {
    unsafe { hal_call!(ptr HAL_InitializeAnalogInputPort(handle)) }
}

pub fn free_analog_input_port(handle: AnalogInputHandle) {
    unsafe { HAL_FreeAnalogInputPort(handle) }
}

pub fn initialize_analog_output_port(handle: PortHandle) -> HalResult<AnalogOutputHandle> {
    unsafe { hal_call!(ptr HAL_InitializeAnalogOutputPort(handle)) }
}

pub fn free_analog_output_port(handle: AnalogOutputHandle) {
    unsafe { HAL_FreeAnalogOutputPort(handle) }
}

// TODO: What does this function do?
pub fn check_analog_module(module: i32) -> bool {
    unsafe { HAL_CheckAnalogModule(module) != 0 }
}

// TODO: What does this function do?
pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}

pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 }
}

pub fn set_analog_sample_rate(samples_per_second: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogSampleRate(samples_per_second)) }
}

pub fn get_analog_sample_rate() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogSampleRate()) }
}

pub fn set_analog_average_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogAverageBits(handle, bits)) }
}

pub fn get_analog_average_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogAverageBits(handle)) }
}

pub fn set_analog_oversample_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogOversampleBits(handle, bits)) }
}

pub fn get_analog_oversample_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogOversampleBits(handle)) }
}

pub fn get_analog_value(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogValue(handle)) }
}

pub fn get_analog_average_value(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogAverageValue(handle)) }
}

pub fn get_analog_volts_to_value(handle: AnalogInputHandle, voltage: f64) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogVoltsToValue(handle, voltage)) }
}

pub fn get_analog_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogVoltage(handle)) }
}

pub fn get_analog_average_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogAverageVoltage(handle)) }
}

pub fn get_analog_lsb_weight(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogLSBWeight(handle)) }
}

pub fn get_analog_offset(handle: AnalogInputHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogOffset(handle)) }
}

pub fn set_analog_output(handle: AnalogOutputHandle, voltage: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogOutput(handle, voltage)) }
}

pub fn get_analog_output(handle: AnalogOutputHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogOutput(handle)) }
}

pub fn is_accumulator_channel(port: AnalogInputHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_IsAccumulatorChannel(port)).map(|n| n != 0) }
}

pub fn initialize_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_InitAccumulator(port)) }
}

pub fn reset_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ResetAccumulator(port)) }
}

pub fn set_accumulator_center(port: AnalogInputHandle, center: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAccumulatorCenter(port, center)) }
}

pub fn set_accumulator_deadband(port: AnalogInputHandle, deadband: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAccumulatorDeadband(port, deadband)) }
}

pub fn get_accumulator_value(port: AnalogInputHandle) -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetAccumulatorValue(port)) }
}


pub fn get_accumulator_count(port: AnalogInputHandle) -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetAccumulatorCount(port)) }
}

pub fn get_accumulator_output(port: AnalogInputHandle) -> HalResult<AccumulatorOutput> {
    let mut value = 0;
    let mut count = 0;

    unsafe {
        hal_call!(ptr HAL_GetAccumulatorOutput(port, &mut value, &mut count))?;
    }

    Ok(AccumulatorOutput { value, count })
}
pub fn initialize_analog_trigger(handle: AnalogInputHandle) -> HalResult<(AnalogTriggerHandle, i32)> {
    let mut index = 0;
    let handle = unsafe { hal_call!(ptr HAL_InitializeAnalogTrigger(handle, &mut index))? };
    Ok((handle, index))
}

// TODO: WHAT DOES THIS DO
pub unsafe fn clean_analog_trigger(handle: AnalogTriggerHandle) -> HalResult<()> {
    hal_call!(ptr HAL_CleanAnalogTrigger(handle))
}

pub unsafe fn set_analog_trigger_limits(handle: AnalogTriggerHandle, limits: AnalogTriggerLimits) -> HalResult<()> {
    match limits {
        AnalogTriggerLimits::Raw(range) => {
            hal_call!(ptr HAL_SetAnalogTriggerLimitsRaw(handle, range.start, range.end))
        }

        AnalogTriggerLimits::Voltage(range) => {
            hal_call!(ptr HAL_SetAnalogTriggerLimitsVoltage(handle, range.start, range.end))
        }
    }
}

pub unsafe fn set_analog_trigger_averaged(handle: AnalogTriggerHandle, use_averaged_value: bool) -> HalResult<()> {
    hal_call!(ptr HAL_SetAnalogTriggerAveraged(handle, use_averaged_value as HAL_Bool))
}

pub unsafe fn set_analog_trigger_filtered(handle: AnalogTriggerHandle, use_filtered_value: bool) -> HalResult<()> {
    hal_call!(ptr HAL_SetAnalogTriggerFiltered(handle, use_filtered_value as HAL_Bool))
}

pub unsafe fn get_analog_trigger_window(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerInWindow(handle)).map(|n| n != 0)
}

pub unsafe fn get_analog_trigger_state(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerTriggerState(handle)).map(|n| n != 0)
}

pub unsafe fn get_analog_trigger_output(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerOutput(handle, trigger_type.into())).map(|n| n != 0)
}
