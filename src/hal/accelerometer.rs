use ::raw::*;

pub type RawAccelerometerRange = HAL_AccelerometerRange;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AccelerometerRange {
    AccelerometerRange2G,
    AccelerometerRange4G,
    AccelerometerRange8G,
}

impl AccelerometerRange {
    pub fn into_raw(&self) -> RawAccelerometerRange {
        match *self {
            AccelerometerRange::AccelerometerRange2G => {
                HAL_AccelerometerRange::HAL_AccelerometerRange_k2G
            }
            AccelerometerRange::AccelerometerRange4G => {
                HAL_AccelerometerRange::HAL_AccelerometerRange_k4G
            }
            AccelerometerRange::AccelerometerRange8G => {
                HAL_AccelerometerRange::HAL_AccelerometerRange_k8G
            }
        }
    }
}

impl From<RawAccelerometerRange> for AccelerometerRange {
    fn from(raw: RawAccelerometerRange) -> Self {
        match raw {
            HAL_AccelerometerRange::HAL_AccelerometerRange_k2G => {
                AccelerometerRange::AccelerometerRange2G
            }
            HAL_AccelerometerRange::HAL_AccelerometerRange_k4G => {
                AccelerometerRange::AccelerometerRange4G
            }
            HAL_AccelerometerRange::HAL_AccelerometerRange_k8G => {
                AccelerometerRange::AccelerometerRange8G
            }
        }
    }
}

pub fn set_accelerometer_active(active: bool) {
    unsafe { HAL_SetAccelerometerActive(active as HAL_Bool) }
}

pub fn set_accelerometer_range(range: AccelerometerRange) {
    unsafe { HAL_SetAccelerometerRange(range.into_raw()) }
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
