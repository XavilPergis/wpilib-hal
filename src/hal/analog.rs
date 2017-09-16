use std::ops::Range;
use std::os::raw::c_double;
use hal::types::{AnalogInputHandle, AnalogOutputHandle, AnalogTriggerHandle, GyroHandle, PortHandle, NativeBool};
use error::*;

extern "C" {
    fn HAL_IsAccumulatorChannel(handle: AnalogInputHandle, status: *mut i32) -> NativeBool;
    fn HAL_InitAccumulator(handle: AnalogInputHandle, status: *mut i32);
    fn HAL_ResetAccumulator(handle: AnalogInputHandle, status: *mut i32);
    fn HAL_SetAccumulatorCenter(handle: AnalogInputHandle, center: i32, status: *mut i32);
    fn HAL_SetAccumulatorDeadband(handle: AnalogInputHandle, deadband: i32, status: *mut i32);
    fn HAL_GetAccumulatorValue(handle: AnalogInputHandle, status: *mut i32) -> i64;
    fn HAL_GetAccumulatorCount(handle: AnalogInputHandle, status: *mut i32) -> i64;
    fn HAL_GetAccumulatorOutput(handle: AnalogInputHandle, value: *mut i64, count: *mut i64, status: *mut i32);
    fn HAL_InitializeAnalogGyro(handle: AnalogInputHandle, status: *mut i32) -> GyroHandle;
    fn HAL_SetupAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_FreeAnalogGyro(handle: GyroHandle);
    fn HAL_SetAnalogGyroParameters(handle: GyroHandle, voltsPerDegreePerSecond: c_double, offset: c_double, center: i32, status: *mut i32);
    fn HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle: GyroHandle, voltsPerDegreePerSecond: c_double, status: *mut i32);
    fn HAL_ResetAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_CalibrateAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_SetAnalogGyroDeadband(handle: GyroHandle, volts: c_double, status: *mut i32);
    fn HAL_GetAnalogGyroAngle(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroRate(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroOffset(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroCenter(handle: GyroHandle, status: *mut i32) -> i32;
    fn HAL_InitializeAnalogInputPort(handle: PortHandle, status: *mut i32) -> AnalogInputHandle;
    fn HAL_FreeAnalogInputPort(handle: AnalogInputHandle);
    fn HAL_CheckAnalogModule(module: i32) -> NativeBool;
    fn HAL_CheckAnalogInputChannel(channel: i32) -> NativeBool;
    fn HAL_SetAnalogSampleRate(samplesPerSecond: c_double, status: *mut i32);
    fn HAL_GetAnalogSampleRate(status: *mut i32) -> c_double;
    fn HAL_SetAnalogAverageBits(handle: AnalogInputHandle, bits: i32, status: *mut i32);
    fn HAL_GetAnalogAverageBits(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_SetAnalogOversampleBits(handle: AnalogInputHandle, bits: i32, status: *mut i32);
    fn HAL_GetAnalogOversampleBits(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_GetAnalogValue(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_GetAnalogAverageValue(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_GetAnalogVoltsToValue(handle: AnalogInputHandle, voltage: c_double, status: *mut i32) -> i32;
    fn HAL_GetAnalogVoltage(handle: AnalogInputHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogAverageVoltage(handle: AnalogInputHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogLSBWeight(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_GetAnalogOffset(handle: AnalogInputHandle, status: *mut i32) -> i32;
    fn HAL_InitializeAnalogOutputPort(handle: PortHandle, status: *mut i32) -> AnalogOutputHandle;
    fn HAL_FreeAnalogOutputPort(handle: AnalogOutputHandle);
    fn HAL_SetAnalogOutput(handle: AnalogOutputHandle, voltage: c_double, status: *mut i32);
    fn HAL_GetAnalogOutput(handle: AnalogOutputHandle, status: *mut i32) -> c_double;
    fn HAL_CheckAnalogOutputChannel(channel: i32) -> NativeBool;
    fn HAL_InitializeAnalogTrigger(handle: AnalogInputHandle, index: *mut i32, status: *mut i32) -> AnalogTriggerHandle;
    fn HAL_CleanAnalogTrigger(handle: AnalogTriggerHandle, status: *mut i32);
    fn HAL_SetAnalogTriggerLimitsRaw(handle: AnalogTriggerHandle, lower: i32, upper: i32, status: *mut i32);
    fn HAL_SetAnalogTriggerLimitsVoltage(handle: AnalogTriggerHandle, lower: c_double, upper: c_double, status: *mut i32);
    fn HAL_SetAnalogTriggerAveraged(handle: AnalogTriggerHandle, useAveragedValue: NativeBool, status: *mut i32);
    fn HAL_SetAnalogTriggerFiltered(handle: AnalogTriggerHandle, useFilteredValue: NativeBool, status: *mut i32);
    fn HAL_GetAnalogTriggerInWindow(handle: AnalogTriggerHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetAnalogTriggerTriggerState(handle: AnalogTriggerHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetAnalogTriggerOutput(handle: AnalogTriggerHandle, type_: AnalogTriggerType, status: *mut i32) -> NativeBool;
}

/// The raw output of an accumulator.
pub struct AccumulatorOutput {
    value: i64,
    count: i64
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AnalogTriggerType {
    /// Makes the trigger return true if the value is in the bounds
    InWindow = 0,
    /// Makes the trigger return the last state if in range, true if higher, or false if lower
    State = 1,
    RisingPulse = 2,
    FallingPulse = 3,
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
    hal_call!(ptr HAL_SetAnalogTriggerAveraged(handle, use_averaged_value as NativeBool))
}

pub unsafe fn set_analog_trigger_filtered(handle: AnalogTriggerHandle, use_filtered_value: bool) -> HalResult<()> {
    hal_call!(ptr HAL_SetAnalogTriggerFiltered(handle, use_filtered_value as NativeBool))
}

pub unsafe fn get_analog_trigger_window(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerInWindow(handle)).map(|n| n != 0)
}

pub unsafe fn get_analog_trigger_state(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerTriggerState(handle)).map(|n| n != 0)
}

pub unsafe fn get_analog_trigger_output(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType) -> HalResult<bool> {
    hal_call!(ptr HAL_GetAnalogTriggerOutput(handle, trigger_type)).map(|n| n != 0)
}
