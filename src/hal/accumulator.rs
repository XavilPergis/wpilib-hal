use super::handle::{AnalogInputHandle, Handle};
use ::error::*;
use ::raw::*;

pub struct Accumulator {
    handle: AnalogInputHandle,
}

impl Accumulator {
    pub fn new(handle: AnalogInputHandle) -> HalResult<Accumulator> {
        self::init_accumulator(handle).map(|_| Accumulator { handle: handle })
    }

    pub fn is_accumulator_channel(handle: AnalogInputHandle) -> HalResult<bool> {
        self::is_accumulator_channel(handle)
    }

    pub fn reset(&self) -> HalResult<()> {
        self::reset_accumulator(self.handle)
    }

    pub fn set_center(&self, center: i32) -> HalResult<()> {
        self::set_accumulator_center(self.handle, center)
    }

    pub fn set_deadband(&self, deadband: i32) -> HalResult<()> {
        self::set_accumulator_deadband(self.handle, deadband)
    }

    pub fn get_value(&self) -> HalResult<i64> {
        self::get_accumulator_value(self.handle)
    }

    pub fn get_count(&self) -> HalResult<i64> {
        self::get_accumulator_count(self.handle)
    }
}

fn is_accumulator_channel(port: AnalogInputHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_IsAccumulatorChannel(port.get_handle()) ].map(|n| n != 0)
}

fn init_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_InitAccumulator(port.get_handle()) ]
}

fn reset_accumulator(port: AnalogInputHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAccumulator(port.get_handle()) ]
}

fn set_accumulator_center(port: AnalogInputHandle, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorCenter(port.get_handle(), center) ]
}

fn set_accumulator_deadband(port: AnalogInputHandle, deadband: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAccumulatorDeadband(port.get_handle(), deadband) ]
}

fn get_accumulator_value(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorValue(port.get_handle()) ]
}

fn get_accumulator_count(port: AnalogInputHandle) -> HalResult<i64> {
    hal_call![ ptr HAL_GetAccumulatorCount(port.get_handle()) ]
}

// TODO: Do we really need this?
// fn HAL_GetAccumulatorOutput(analogPortHandle: HAL_AnalogInputHandle,
//                                value: *mut i64,
//                                count: *mut i64,
//                                status: *mut i32);
