use ::raw::*;

#[allow(missing_docs)]
pub type RawAccelerometerRange = HAL_AccelerometerRange;

/// The range of g-force the accelerometer will output. `Max4G` means that it will
/// output a maximum value of 4g
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AccelerometerRange {
    /// 8g maximum output
    Max8G,
    /// 4g maximum output
    Max4G,
    /// 2g maximum output
    Max2G,
}

impl_convert! {
    HAL_AccelerometerRange, AccelerometerRange;
    HAL_AccelerometerRange_k2G <=> Max2G,
    HAL_AccelerometerRange_k4G <=> Max4G,
    HAL_AccelerometerRange_k8G <=> Max8G
}

/// Set the accelerometer to active or standby mode. It must be in standby
/// mode to change any configuration.
pub unsafe fn set_accelerometer_active(active: bool) {
    HAL_SetAccelerometerActive(active as HAL_Bool)
}

/// Set the range of values that can be measured (either 2, 4, or 8 g-forces).
/// The accelerometer should be in standby mode when this is called.
pub unsafe fn set_accelerometer_range(range: AccelerometerRange) {
    HAL_SetAccelerometerRange(range.into())
}

/// Gets the acceleromenter's X (Left/right) value. Returns a value in units of 1 g-force.
pub unsafe fn get_accelerometer_x() -> f64 {
    HAL_GetAccelerometerX()
}

/// Gets the acceleromenter's Y (Front/back) value. Returns a value in units of 1 g-force.
pub unsafe fn get_accelerometer_y() -> f64 {
    HAL_GetAccelerometerY()
}

/// Gets the acceleromenter's Z (Top/bottom) value. Returns a value in units of 1 g-force.
pub unsafe fn get_accelerometer_z() -> f64 {
    HAL_GetAccelerometerZ()
}
