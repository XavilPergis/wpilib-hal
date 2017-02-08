use ::raw::*;
use hal::handle::*;
use ::error::*;
use hal::analog_input::AnalogInput;

pub struct AnalogGyro {
    port: GyroHandle,
}

impl AnalogGyro {
    pub fn new(port: AnalogInputHandle) -> HalResult<AnalogGyro> {
        self::initialize_analog_gyro(port).map(|gh| AnalogGyro { port: gh })
    }
}

impl AnalogInput for AnalogGyro {
    fn get_handle(&self) -> AnalogInputHandle {
        *self.port.transmute_handle::<AnalogInputHandle>()
    }
}

pub fn initialize_analog_gyro(port: AnalogInputHandle) -> HalResult<GyroHandle> {
    hal_call![ ptr HAL_InitializeAnalogGyro(port.get_handle()) ].map(|n| GyroHandle(n))
}

pub fn setup_analog_gyro(port: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetupAnalogGyro(port.get_handle()) ]
}

pub fn free_analog_gyro(port: GyroHandle) {
    unsafe { HAL_FreeAnalogGyro(port.get_handle()) }
}

pub fn set_analog_gyro_parameters(handle: GyroHandle,
                                  volts_per_degree_per_second: f64,
                                  offset: f64,
                                  center: i32)
                                  -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroParameters(handle.get_handle(), volts_per_degree_per_second, offset, center) ]
}

pub fn set_analog_gyro_volts_per_degree_per_second(handle: GyroHandle, vds: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle.get_handle(), vds) ]
}

pub fn reset_analog_gyro(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetAnalogGyro(handle.get_handle()) ]
}

pub fn calibrate_analog_gyro(handle: GyroHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CalibrateAnalogGyro(handle.get_handle()) ]
}

pub fn set_analog_gyro_deadband(handle: GyroHandle, volts: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetAnalogGyroDeadband(handle.get_handle(), volts) ]
}

pub fn get_analog_gyro_angle(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroAngle(handle.get_handle()) ]
}

pub fn get_analog_gyro_rate(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroRate(handle.get_handle()) ]
}

pub fn get_analog_gyro_offset(handle: GyroHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetAnalogGyroOffset(handle.get_handle()) ]
}

pub fn get_analog_gyro_center(handle: GyroHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAnalogGyroCenter(handle.get_handle()) ]
}
