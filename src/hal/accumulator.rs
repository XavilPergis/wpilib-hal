use ::error::*;
use ::raw::*;
use handle::*;

/// The raw output of an accumulator.
pub struct AccumulatorOutput {
    value: i64,
    count: i64
}

pub unsafe fn is_accumulator_channel(port: AnalogInputHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_IsAccumulatorChannel(port) ].map(|n| n != 0)
}

pub unsafe fn initialize(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_InitAccumulator(port) ]
}

pub unsafe fn reset(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAccumulator(port) ]
}

pub unsafe fn set_center(port: AnalogInputHandle, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorCenter(port, center) ]
}

pub unsafe fn set_deadband(port: AnalogInputHandle, deadband: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorDeadband(port, deadband) ]
}

pub unsafe fn get_value(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorValue(port) ]
}

pub unsafe fn get_accumulator_count(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorCount(port) ]
}

pub unsafe fn get_output(port: AnalogInputHandle) -> HalResult<AccumulatorOutput> {
    let mut value = 0;
    let mut count = 0;

    // Load output into vars
    hal_call![ ptr HAL_GetAccumulatorOutput(port, &mut value, &mut count) ]?;

    Ok(AccumulatorOutput { value, count })
}
