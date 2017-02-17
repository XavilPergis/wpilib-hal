use ::error::*;
use hal::handle::*;
use ::raw::*;

pub fn initialize(port: AnalogInputHandle) -> HalResult<GyroHandle> {
    hal_call![ ptr HAL_InitializeAnalogGyro(port) ]
}

pub fn setup(port: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetupAnalogGyro(port) ]
}

pub fn free(port: GyroHandle) {
    unsafe { HAL_FreeAnalogGyro(port) }
}

pub fn set_parameters(handle: GyroHandle, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroParameters(handle, volts_per_degree_per_second, offset, center) ]
}

pub fn set_volts_per_degree_per_second(handle: GyroHandle, vds: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle, vds) ]
}

pub fn reset(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAnalogGyro(handle) ]
}

pub fn calibrate(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CalibrateAnalogGyro(handle) ]
}

pub fn set_deadband(handle: GyroHandle, volts: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroDeadband(handle, volts) ]
}

pub fn get_angle(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroAngle(handle) ]
}

pub fn get_rate(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroRate(handle) ]
}

pub fn get_offset(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroOffset(handle) ]
}

pub fn get_center(handle: GyroHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogGyroCenter(handle) ]
}
