use ::error::*;
use hal::handle::*;
use ::raw::*;

pub unsafe fn initialize_analog_input_port(handle: PortHandle) -> HalResult<AnalogInputHandle> {
    hal_call![ ptr HAL_InitializeAnalogInputPort(handle) ]
}

pub unsafe fn free_analog_input_port(handle: AnalogInputHandle) {
    HAL_FreeAnalogInputPort(handle)
}

// TODO: What does this function do?
pub unsafe fn check_analog_module(module: i32) -> bool {
    HAL_CheckAnalogModule(module) != 0
}

// TODO: What does this function do?
pub unsafe fn check_analog_input_channel(channel: i32) -> bool {
    HAL_CheckAnalogInputChannel(channel) != 0
}

pub unsafe fn set_analog_sample_rate(samples_per_second: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogSampleRate(samples_per_second) ]
}

pub unsafe fn get_analog_sample_rate() -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogSampleRate() ]
}

pub unsafe fn set_analog_average_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogAverageBits(handle, bits) ]
}

pub unsafe fn get_analog_average_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogAverageBits(handle) ]
}

pub unsafe fn set_analog_oversample_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogOversampleBits(handle, bits) ]
}

pub unsafe fn get_analog_oversample_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogOversampleBits(handle) ]
}

pub unsafe fn get_analog_value(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogValue(handle) ]
}

pub unsafe fn get_analog_average_value(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogAverageValue(handle) ]
}

pub unsafe fn get_analog_volts_to_value(handle: AnalogInputHandle, voltage: f64) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogVoltsToValue(handle, voltage) ]
}

pub unsafe fn get_analog_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogVoltage(handle) ]
}

pub unsafe fn get_analog_average_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogAverageVoltage(handle) ]
}

pub unsafe fn get_analog_lsb_weight(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogLSBWeight(handle) ]
}

pub unsafe fn get_analog_offset(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogOffset(handle) ]
}
