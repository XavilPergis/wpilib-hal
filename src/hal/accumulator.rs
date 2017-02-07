use ::raw::*;

pub fn is_accumulator_channel(port: AnalogInputHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_IsAccumulatorChannel(port.get_handle()) ].map(|n| n != 0)
}

pub fn init_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_InitAccumulator(port.get_handle()) ]
}

pub fn reset_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAccumulator(port.get_handle()) ]
}

pub fn set_accumulator_center(port: AnalogInputHandle, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorCenter(port.get_handle(), center) ]
}

pub fn set_accumulator_deadband(port: AnalogInputHandle, deadband: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorDeadband(port.get_handle(), deadband) ]
}

pub fn get_accumulator_value(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorValue(port.get_handle()) ]
}

pub fn get_accumulator_count(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorCount(port.get_handle()) ]
}

// TODO: Do we really need this?
//pub fn HAL_GetAccumulatorOutput(analogPortHandle: HAL_AnalogInputHandle,
//                                value: *mut i64,
//                                count: *mut i64,
//                                status: *mut i32);
