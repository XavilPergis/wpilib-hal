use ::error::*;
use hal::handle::*;
use ::raw::*;

pub trait AnalogOutput {
    fn get(&self) -> HalResult<f64>;
    fn set(&mut self) -> HalResult<()>;
}

pub fn initialize_analog_output_port(handle: PortHandle) -> HalResult<AnalogOutputHandle> {
    hal_call![ ptr HAL_InitializeAnalogOutputPort(handle) ]
}

pub fn free_analog_output_port(handle: AnalogOutputHandle) {
    unsafe { HAL_FreeAnalogOutputPort(handle) }
}

pub fn set_analog_output(handle: AnalogOutputHandle, voltage: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogOutput(handle, voltage) ]
}

pub fn get_analog_output(handle: AnalogOutputHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogOutput(handle) ]
}

pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 01 }
}
