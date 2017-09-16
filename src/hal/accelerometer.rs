use hal::types::NativeBool;
use std::os::raw::c_double;

extern "C" {
    fn HAL_SetAccelerometerActive(active: NativeBool);
    fn HAL_SetAccelerometerRange(range: AccelerometerRange);
    fn HAL_GetAccelerometerX() -> c_double;
    fn HAL_GetAccelerometerY() -> c_double;
    fn HAL_GetAccelerometerZ() -> c_double;
}

/// The range of g-force the accelerometer will output. `Max4G` means that it will
/// output a maximum value of 4g
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AccelerometerRange {
    /// 8g maximum output
    Max8G = 0,
    /// 4g maximum output
    Max4G = 1,
    /// 2g maximum output
    Max2G = 2,
}

/// Set the accelerometer to active or standby mode. It must be in standby
/// mode to change any configuration.
pub fn set_accelerometer_active(active: bool) {
    unsafe { HAL_SetAccelerometerActive(active as NativeBool) }
}

/// Set the range of values that can be measured (either 2, 4, or 8 g-forces).
/// The accelerometer should be in standby mode when this is called.
pub fn set_accelerometer_range(range: AccelerometerRange) {
    unsafe { HAL_SetAccelerometerRange(range) }
}

/// Gets the acceleromenter's X (Left/right) value. Returns a value in units of 1 g-force.
pub fn get_accelerometer_x() -> f64 {
    unsafe { HAL_GetAccelerometerX() }
}

/// Gets the acceleromenter's Y (Front/back) value. Returns a value in units of 1 g-force.
pub fn get_accelerometer_y() -> f64 {
    unsafe { HAL_GetAccelerometerY() }
}

/// Gets the acceleromenter's Z (Top/bottom) value. Returns a value in units of 1 g-force.
pub fn get_accelerometer_z() -> f64 {
    unsafe { HAL_GetAccelerometerZ() }
}
