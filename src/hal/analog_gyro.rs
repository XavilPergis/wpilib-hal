use std::os::raw::c_double;
use hal::types::{AnalogInputHandle, GyroHandle};
use error::*;

extern "C" {
    fn HAL_InitializeAnalogGyro(handle: AnalogInputHandle,
                                    status: *mut i32) -> GyroHandle;
    fn HAL_SetupAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_FreeAnalogGyro(handle: GyroHandle);
    fn HAL_SetAnalogGyroParameters(handle: GyroHandle,
                                       voltsPerDegreePerSecond: c_double,
                                       offset: c_double,
                                       center: i32, status: *mut i32);
    fn HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle: GyroHandle,
                                                    voltsPerDegreePerSecond: c_double,
                                                    status: *mut i32);
    fn HAL_ResetAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_CalibrateAnalogGyro(handle: GyroHandle, status: *mut i32);
    fn HAL_SetAnalogGyroDeadband(handle: GyroHandle,
                                     volts: c_double,
                                     status: *mut i32);
    fn HAL_GetAnalogGyroAngle(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroRate(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroOffset(handle: GyroHandle, status: *mut i32) -> c_double;
    fn HAL_GetAnalogGyroCenter(handle: GyroHandle, status: *mut i32) -> i32;
}

pub fn initialize(port: AnalogInputHandle) -> HalResult<GyroHandle> {
    unsafe { hal_call!(ptr HAL_InitializeAnalogGyro(port)) }
}

pub fn setup(port: GyroHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetupAnalogGyro(port)) }
}

pub fn free(port: GyroHandle) {
    unsafe { HAL_FreeAnalogGyro(port) }
}

pub fn set_parameters(handle: GyroHandle, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogGyroParameters(handle, volts_per_degree_per_second, offset, center)) }
}

pub fn set_volts_per_degree_per_second(handle: GyroHandle, vds: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle, vds)) }
}

pub fn reset(handle: GyroHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ResetAnalogGyro(handle)) }
}

pub fn calibrate(handle: GyroHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_CalibrateAnalogGyro(handle)) }
}

pub fn set_deadband(handle: GyroHandle, volts: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAnalogGyroDeadband(handle, volts)) }
}

pub fn get_angle(handle: GyroHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogGyroAngle(handle)) }
}

pub fn get_rate(handle: GyroHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogGyroRate(handle)) }
}

pub fn get_offset(handle: GyroHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetAnalogGyroOffset(handle)) }
}

pub fn get_center(handle: GyroHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAnalogGyroCenter(handle)) }
}
