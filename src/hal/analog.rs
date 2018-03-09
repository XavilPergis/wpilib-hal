use std::os::raw::c_double;
use hal::types::{AnalogInputHandle, AnalogOutputHandle, GyroHandle, PortHandle, NativeBool};
use error::*;

extern "C" {
    fn HAL_InitAccumulator(handle: AnalogInputHandle, status: *mut i32);
    fn HAL_ResetAccumulator(handle: AnalogInputHandle, status: *mut i32);
    fn HAL_IsAccumulatorChannel(handle: AnalogInputHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetAccumulatorCenter(handle: AnalogInputHandle, center: i32, status: *mut i32);
    fn HAL_SetAccumulatorDeadband(handle: AnalogInputHandle, deadband: i32, status: *mut i32);
    fn HAL_GetAccumulatorValue(handle: AnalogInputHandle, status: *mut i32) -> i64;
    fn HAL_GetAccumulatorCount(handle: AnalogInputHandle, status: *mut i32) -> i64;
    fn HAL_GetAccumulatorOutput(handle: AnalogInputHandle, value: *mut i64, count: *mut i64, status: *mut i32);

    fn HAL_InitializeAnalogInputPort(handle: PortHandle, status: *mut i32) -> AnalogInputHandle;
    fn HAL_FreeAnalogInputPort(handle: AnalogInputHandle);
    // fn HAL_CheckAnalogModule(module: i32) -> NativeBool;
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
        if !check_input_channel(channel) { return Err(HalError::InvalidChannel(channel)); }

        let port_handle = ::hal::get_port(channel).ok_or(HalError::InvalidChannel(channel))?;
        let port = unsafe { hal_call!(HAL_InitializeAnalogInputPort(port_handle))? };
        
        Ok(AnalogInput { port, channel })
    }

    pub fn set_oversample_bits(&self, bits: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogOversampleBits(self.port, bits)) }
    }
    
    /// Set the global sample rate for all DigitalInputs in samples per second
    pub fn set_sample_rate(sample_rate: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogSampleRate(sample_rate)) }
    }
    
    pub fn get_sample_rate() -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogSampleRate()) }
    }
    
    /// Set the size of the averaging window. The sampling window can only be sized in powers
    /// of 2, so the actual number of samples in a window is `2^bits`
    pub fn set_average_bits(&self, bits: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogAverageBits(self.port, bits)) }
    }
    
    pub fn get_average_bits(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogAverageBits(self.port)) }
    }
    
    pub fn get_oversample_bits(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogOversampleBits(self.port)) }
    }
    
    pub fn get_value(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogValue(self.port)) }
    }
    
    pub fn get_average_value(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogAverageValue(self.port)) }
    }
    
    pub fn get_volts_to_value(&self, voltage: f64) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogVoltsToValue(self.port, voltage)) }
    }
    
    pub fn get_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogVoltage(self.port)) }
    }
    
    pub fn get_average_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogAverageVoltage(self.port)) }
    }
    
    pub fn get_lsb_weight(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogLSBWeight(self.port)) }
    }
    
    pub fn get_offset(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogOffset(self.port) ) }
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
        if !check_output_channel(channel) { return Err(HalError::InvalidChannel(channel)); }

        let port_handle = ::hal::get_port(channel).ok_or(HalError::InvalidChannel(channel))?;
        let port = unsafe { hal_call!(HAL_InitializeAnalogOutputPort(port_handle))? };
        
        Ok(AnalogOutput { port, channel })
    }

    pub fn set_voltage(&self, voltage: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogOutput(self.port, voltage)) }
    }

    pub fn get_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogOutput(self.port)) }
    }
}

impl Drop for AnalogOutput {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogOutputPort(self.port); }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct AccumulatorOutput {
    pub value: i64,
    pub count: i64,
}

#[derive(Debug)]
pub struct AnalogAccumulator<'i> {
    input: &'i AnalogInput,
    // The constant in a definite integral
    offset: i64,
}

use std::time::Duration;

impl<'i> AnalogAccumulator<'i> {
    /// An `InvalidChannel` error means that the channel was not an accumulator channel.
    fn new(input: &'i AnalogInput, offset: i64) -> HalResult<Self> {
        unsafe {
            if hal_call!(HAL_IsAccumulatorChannel(input.port))? != 0 {
                return Err(HalError::InvalidChannel(input.port));
            }

            hal_call!(HAL_InitAccumulator(input.port))?;
        }

        Ok(AnalogAccumulator { input, offset })
    }

    pub fn sample_period(&self) -> HalResult<Duration> {
        let sample_time = 1.0 / AnalogInput::get_sample_rate()?;
        let over_samples = 1 << self.input.get_oversample_bits()?;
        let average_samples = 1 << self.input.get_average_bits()?;
        let nanos = sample_time * over_samples as f64 * average_samples as f64 * 1_000_000_000.0;
        Ok(Duration::new(0, nanos as u32))
    } 

    pub fn reset(&self) -> HalResult<()> {
        // Reset the accumulator and then sleep for a period so we don't query old values
        // on the next call
        unsafe { hal_call!(HAL_ResetAccumulator(self.input.port))?; }
        ::std::thread::sleep(self.sample_period()?);
        Ok(())
    }

    pub fn set_center(&self, center: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAccumulatorCenter(self.input.port, center)) }
    }

    pub fn set_deadband(&self, deadband: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAccumulatorDeadband(self.input.port, deadband)) }
    }

    pub fn get_value(&self) -> HalResult<i64> {
        unsafe { hal_call!(HAL_GetAccumulatorValue(self.input.port)).map(|val| val + self.offset) }
    }

    pub fn get_count(&self) -> HalResult<i64> {
        unsafe { hal_call!(HAL_GetAccumulatorCount(self.input.port)) }
    }

    pub fn get_output(&self) -> HalResult<AccumulatorOutput> {
        let mut output = AccumulatorOutput::default();
        unsafe { hal_call!(HAL_GetAccumulatorOutput(self.input.port, &mut output.value, &mut output.count))?; }
        Ok(output)
    }

}
