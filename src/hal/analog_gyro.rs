use ::error::*;
use hal::handle::*;
use ::raw::*;

pub unsafe fn initialize(port: AnalogInputHandle) -> HalResult<GyroHandle> {
    hal_call![ ptr HAL_InitializeAnalogGyro(port) ]
}

pub unsafe fn setup(port: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetupAnalogGyro(port) ]
}

pub unsafe fn free(port: GyroHandle) {
    HAL_FreeAnalogGyro(port)
}

pub unsafe fn set_parameters(handle: GyroHandle, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroParameters(handle, volts_per_degree_per_second, offset, center) ]
}

pub unsafe fn set_volts_per_degree_per_second(handle: GyroHandle, vds: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle, vds) ]
}

pub unsafe fn reset(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAnalogGyro(handle) ]
}

pub unsafe fn calibrate(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CalibrateAnalogGyro(handle) ]
}

pub unsafe fn set_deadband(handle: GyroHandle, volts: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroDeadband(handle, volts) ]
}

pub unsafe fn get_angle(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroAngle(handle) ]
}

pub unsafe fn get_rate(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroRate(handle) ]
}

pub unsafe fn get_offset(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroOffset(handle) ]
}

pub unsafe fn get_center(handle: GyroHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogGyroCenter(handle) ]
}
