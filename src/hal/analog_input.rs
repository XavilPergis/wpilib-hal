use ::raw::*;

// TODO: rait or struct?
pub trait AnalogInput {
    fn get_handle(&self) -> AnalogInputHandle;

    fn get_raw(&self) -> HalResult<i32> {
        self::get_analog_value(self.get_handle())
    }

    fn get_voltage(&self) -> HalResult<f64> {
        self::get_analog_voltage(self.get_handle())
    }

    fn get_average_raw(&self) -> HalResult<i32> {
        self::get_analog_average_value(self.get_handle())
    }

    fn get_average_voltage(&self) -> HalResult<f64> {
        self::get_analog_average_voltage(self.get_handle())
    }
}

pub struct IrSensor {
    handle: AnalogInputHandle
}

impl AnalogInput for IrSensor {
    fn get_handle(&self) -> AnalogInputHandle {
        self.handle
    }
}

pub fn initialize_analog_input_port(handle: PortHandle) -> HalResult<AnalogInputHandle> {
    hal_call![ ptr HAL_InitializeAnalogInputPort(handle.get_handle()) ].map(|n| AnalogInputHandle(n))
}

pub fn free_analog_input_port(handle: AnalogInputHandle) {
    unsafe { HAL_FreeAnalogInputPort(handle.get_handle()) }
}

// What the fuck is this
pub fn check_analog_module(module: i32) -> bool {
    unsafe { HAL_CheckAnalogModule(module) != 0 }
}

// Also what the fuck
pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}

pub fn set_analog_sample_rate(samples_per_second: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogSampleRate(samples_per_second) ]
}

pub fn get_analog_sample_rate() -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogSampleRate() ]
}

pub fn set_analog_average_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogAverageBits(handle.get_handle(), bits) ]
}

pub fn get_analog_average_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogAverageBits(handle.get_handle()) ]
}

pub fn set_analog_oversample_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogOversampleBits(handle.get_handle(), bits) ]
}

pub fn get_analog_oversample_bits(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogOversampleBits(handle.get_handle()) ]
}

pub fn get_analog_value(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogValue(handle.get_handle()) ]
}

pub fn get_analog_average_value(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogAverageValue(handle.get_handle()) ]
}

pub fn get_analog_volts_to_value(handle: AnalogInputHandle, voltage: f64) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogVoltsToValue(handle.get_handle(), voltage) ]
}

pub fn get_analog_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogVoltage(handle.get_handle()) ]
}

pub fn get_analog_average_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogAverageVoltage(handle.get_handle()) ]
}

pub fn get_analog_lsb_weight(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogLSBWeight(handle.get_handle()) ]
}

pub fn get_analog_offset(handle: AnalogInputHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogOffset(handle.get_handle()) ]
}

