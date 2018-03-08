use std::os::raw::c_double;
use hal::types::{AnalogInputHandle, GyroHandle};
use hal::analog::AnalogInput;
use error::*;

extern "C" {
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
}

#[derive(Debug)]
pub struct AnalogGyro<'i> {
    _channel: &'i AnalogInput,
    handle: GyroHandle
}

impl<'i> AnalogGyro<'i> {
    pub fn new(channel: &'i AnalogInput) -> HalResult<Self> {
        unsafe {
            let handle = hal_call!(HAL_InitializeAnalogGyro(channel.port))?;
            hal_call!(HAL_SetupAnalogGyro(handle))?;
            hal_call!(HAL_CalibrateAnalogGyro(handle))?;
            Ok(AnalogGyro { _channel: channel, handle })
        }
    }

    // TODO: is it possible to change configuration after initialization
    pub fn with_parameters(channel: &'i AnalogInput, sensitivity: f64, offset: f64, center: i32) -> HalResult<Self> {
        let gyro = AnalogGyro::new(channel)?;
        unsafe { hal_call!(HAL_SetAnalogGyroParameters(gyro.handle, sensitivity, offset, center))?; }
        gyro.reset()?;
        Ok(gyro)
    }

    /// Sensitivity in volts/degree/second
    pub fn set_sensitivity(&self, vds: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogGyroVoltsPerDegreePerSecond(self.handle, vds)) }
    }
    
    pub fn reset(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_ResetAnalogGyro(self.handle)) }
    }
    
    pub fn calibrate(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_CalibrateAnalogGyro(self.handle)) }
    }
    
    pub fn set_deadband(&self, volts: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogGyroDeadband(self.handle, volts)) }
    }
    
    pub fn get_angle(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogGyroAngle(self.handle)) }
    }
    
    pub fn get_rate(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogGyroRate(self.handle)) }
    }
    
    pub fn get_offset(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetAnalogGyroOffset(self.handle)) }
    }
    
    pub fn get_center(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetAnalogGyroCenter(self.handle)) }
    }
}

impl<'i> Drop for AnalogGyro<'i> {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogGyro(self.handle); }
    }
}
