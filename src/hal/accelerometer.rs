use ::raw::*;

pub type RawAccelerometerRange = HAL_AccelerometerRange;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AccelerometerRange {
    AccelerometerRange2G,
    AccelerometerRange4G,
    AccelerometerRange8G
}

impl From<RawAccelerometerRange> for AccelerometerRange {
    fn from(raw: RawAccelerometerRange) -> Self {
        match raw {
            RawAccelerometerRange::HAL_AccelerometerRange_k2G => AccelerometerRange::AccelerometerRange2G,
            RawAccelerometerRange::HAL_AccelerometerRange_k4G => AccelerometerRange::AccelerometerRange4G,
            RawAccelerometerRange::HAL_AccelerometerRange_k8G => AccelerometerRange::AccelerometerRange8G,
        }
    }
}

pub fn set_accelerometer_active(active: bool) {
    unsafe { HAL_SetAccelerometerActive(active as HAL_Bool) }
}

pub fn set_accelerometer_range<T>(range: T) where T: Into<AccelerometerRange> {
    unsafe { HAL_SetAccelerometerRange(range.into()) }
}

pub fn get_accelerometer_x() -> f64 {
    unsafe { HAL_GetAccelerometerX() }
}

pub fn get_accelerometer_y() -> f64 {
    unsafe { HAL_GetAccelerometerY() }
}

pub fn get_accelerometer_z() -> f64 {
    unsafe { HAL_GetAccelerometerZ() }
}