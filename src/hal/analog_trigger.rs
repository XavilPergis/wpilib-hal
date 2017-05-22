//! An analog trigger takes some analog input and can be checked to see if the output is in a
//! certain range.

use ::error::*;
use hal::handle::*;
use ::raw::*;
use std::ops::Range;

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

pub unsafe fn initialize(handle: AnalogInputHandle, index: &mut i32) -> HalResult<AnalogTriggerHandle> {
    hal_call![ ptr HAL_InitializeAnalogTrigger(handle, index) ]
}

/// TODO: WHAT DOES THIS DO
pub unsafe fn clean(handle: AnalogTriggerHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanAnalogTrigger(handle) ]
}

/// Set the bounds of trigger
///
/// ### Example
/// ```rust,no_run
/// // Set the range the values have to be between to activate the trigger
/// analog_trigger::set_limits(handle, AnalogTriggerLimits::Voltage(0.5 .. 1.0))?;
/// println!("{}", trigger.in_window()?);
/// ```
pub unsafe fn set_limits(handle: AnalogTriggerHandle, limits: AnalogTriggerLimits) -> HalResult<()> {
    match limits {
        AnalogTriggerLimits::Raw(range) => {
            hal_call![ ptr HAL_SetAnalogTriggerLimitsRaw(handle, range.start, range.end) ]
        }

        AnalogTriggerLimits::Voltage(range) => {
            hal_call![ ptr HAL_SetAnalogTriggerLimitsVoltage(handle, range.start, range.end) ]
        }
    }
}

/// Set whether the input is averaged over a few samples or not
pub unsafe fn set_averaged(handle: AnalogTriggerHandle, use_averaged_value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerAveraged(handle, use_averaged_value as HAL_Bool) ]
}

/// From [Screensteps Live](http://wpilib.screenstepslive.com/s/3120/m/7912/l/85776-analog-triggers)
///
/// The filtering option of the analog trigger uses a 3-point average reject filter. This
/// filter uses a circular buffer of the last three data points and selects the outlier point
/// nearest the median as the output. The primary use of this filter is to reject datapoints
/// which errantly (due to averaging or sampling) appear within the window when detecting
/// transitions using the Rising Edge and Falling Edge functionality of the analog trigger.
pub unsafe fn set_filtered(handle: AnalogTriggerHandle, use_filtered_value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogTriggerFiltered(handle, use_filtered_value as HAL_Bool) ]
}

pub unsafe fn in_window(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerInWindow(handle) ].map(|n| n != 0)
}

pub unsafe fn get_state(handle: AnalogTriggerHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerTriggerState(handle) ].map(|n| n != 0)
}

pub unsafe fn get_output(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType) -> HalResult<bool> {
    hal_call![ ptr HAL_GetAnalogTriggerOutput(handle, trigger_type.into()) ].map(|n| n != 0)
}

pub unsafe fn if_activated<F>(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType, func: F) where F: Fn() {
    match get_output(handle, trigger_type) {
        Ok(true) => func(),
        _ => {}
    }
}
