use std::os::raw::c_double;
use hal::types::{AnalogInputHandle, AnalogOutputHandle, GyroHandle, PortHandle, NativeBool};
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
    fn HAL_SetAnalogOversampleBits(handle: AnalogInputHandle, bits: i32, status: *mut i32);
    fn HAL_SetAnalogSampleRate(samplesPerSecond: c_double, status: *mut i32);
    fn HAL_SetAnalogAverageBits(handle: AnalogInputHandle, bits: i32, status: *mut i32);
    fn HAL_GetAnalogSampleRate(status: *mut i32) -> c_double;
    fn HAL_GetAnalogAverageBits(handle: AnalogInputHandle, status: *mut i32) -> i32;
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
    fn HAL_CheckAnalogOutputChannel(channel: i32) -> NativeBool;
    fn HAL_SetAnalogOutput(handle: AnalogOutputHandle, voltage: c_double, status: *mut i32);
    fn HAL_GetAnalogOutput(handle: AnalogOutputHandle, status: *mut i32) -> c_double;
}

fn check_output_channel(channel: i32) -> bool { unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 } }
fn check_input_channel(channel: i32) -> bool { unsafe { HAL_CheckAnalogInputChannel(channel) != 0 } }

#[derive(Debug)]
pub struct AnalogInput {
    pub(crate) port: i32,
    pub(crate) channel: i32,
}

impl AnalogInput {
    pub fn new(channel: i32) -> HalResult<Self> {
        if !check_input_channel(channel) { return Err(HalError::OutOfRange); }

        let port_handle = ::hal::get_port(channel).ok_or(HalError::OutOfRange)?;
        let port = unsafe { hal_call!(ptr HAL_InitializeAnalogInputPort(port_handle))? };
        
        Ok(AnalogInput { port, channel })
    }

    fn set_analog_oversample_bits(&self, bits: i32) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetAnalogOversampleBits(self.port, bits)) }
    }
    
    /// Set the global sample rate for all DigitalInputs in samples per second
    fn set_analog_sample_rate(sample_rate: f64) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetAnalogSampleRate(sample_rate)) }
    }
    
    fn get_analog_sample_rate() -> HalResult<f64> {
        unsafe { hal_call!(ptr HAL_GetAnalogSampleRate()) }
    }
    
    /// Set the size of the averaging window. The sampling window can only be sized in powers
    /// of 2, so the actual number of samples in a window is `2^bits`
    fn set_analog_average_bits(&self, bits: i32) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetAnalogAverageBits(self.port, bits)) }
    }
    
    fn get_analog_average_bits(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogAverageBits(self.port)) }
    }
    
    fn get_analog_oversample_bits(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogOversampleBits(self.port)) }
    }
    
    fn get_analog_value(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogValue(self.port)) }
    }
    
    fn get_analog_average_value(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogAverageValue(self.port)) }
    }
    
    fn get_analog_volts_to_value(&self, voltage: f64) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogVoltsToValue(self.port, voltage)) }
    }
    
    fn get_analog_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(ptr HAL_GetAnalogVoltage(self.port)) }
    }
    
    fn get_analog_average_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(ptr HAL_GetAnalogAverageVoltage(self.port)) }
    }
    
    fn get_analog_lsb_weight(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogLSBWeight(self.port)) }
    }
    
    fn get_analog_offset(&self) -> HalResult<i32> {
        unsafe { hal_call!(ptr HAL_GetAnalogOffset(self.port) ) }
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogInputPort(self.port); }
    }
}

#[derive(Debug)]
pub struct AnalogOutput {
    port: i32,
    channel: i32,
}

impl AnalogOutput {
    pub fn new(channel: i32) -> HalResult<Self> {
        if !check_output_channel(channel) { return Err(HalError::OutOfRange); }

        let port_handle = ::hal::get_port(channel).ok_or(HalError::OutOfRange)?;
        let port = unsafe { hal_call!(ptr HAL_InitializeAnalogOutputPort(port_handle))? };
        
        Ok(AnalogOutput { port, channel })
    }

    pub fn set_voltage(&self, voltage: f64) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetAnalogOutput(self.port, voltage)) }
    }

    pub fn get_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(ptr HAL_GetAnalogOutput(self.port)) }
    }
}

impl Drop for AnalogOutput {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogOutputPort(self.port); }
    }
}

// TODO
#[derive(Debug)]
pub struct AnalogIO {
    input: AnalogInput,
    output: AnalogOutput,
}
