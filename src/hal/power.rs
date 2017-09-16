use ::error::*;
use ::raw::*;

pub fn get_vin_voltage() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetVinVoltage()) }
}

pub fn get_vin_current() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetVinCurrent()) }
}

pub fn get_user_voltage6v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage6V()) }
}

pub fn get_user_current6v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent6V()) }
}

pub fn get_user_active6v() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive6V()).map(|n| n != 0) }
}

pub fn get_user_current_faults6v() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults6V()) }
}

pub fn get_user_voltage5v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage5V()) }
}

pub fn get_user_current5v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent5V()) }
}

pub fn get_user_active5v() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive5V()).map(|n| n != 0) }
}

pub fn get_user_current_faults5v() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults5V()) }
}

pub fn get_user_voltage3v3() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage3V3()) }
}

pub fn get_user_current3v3() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent3V3()) }
}

pub fn get_user_active3v3() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive3V3()).map(|n| n != 0) }
}

pub fn get_user_current_faults3v3() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults3V3()) }
}
