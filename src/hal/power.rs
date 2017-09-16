use std::os::raw::c_double;
use hal::types::NativeBool;
use error::*;

extern "C" {
    fn HAL_GetVinVoltage(status: *mut i32) -> c_double;
    fn HAL_GetVinCurrent(status: *mut i32) -> c_double;
    fn HAL_GetUserVoltage6V(status: *mut i32) -> c_double;
    fn HAL_GetUserCurrent6V(status: *mut i32) -> c_double;
    fn HAL_GetUserActive6V(status: *mut i32) -> NativeBool;
    fn HAL_GetUserCurrentFaults6V(status: *mut i32) -> i32;
    fn HAL_GetUserVoltage5V(status: *mut i32) -> c_double;
    fn HAL_GetUserCurrent5V(status: *mut i32) -> c_double;
    fn HAL_GetUserActive5V(status: *mut i32) -> NativeBool;
    fn HAL_GetUserCurrentFaults5V(status: *mut i32) -> i32;
    fn HAL_GetUserVoltage3V3(status: *mut i32) -> c_double;
    fn HAL_GetUserCurrent3V3(status: *mut i32) -> c_double;
    fn HAL_GetUserActive3V3(status: *mut i32) -> NativeBool;
    fn HAL_GetUserCurrentFaults3V3(status: *mut i32) -> i32;
}

#[inline(always)]
pub fn get_vin_voltage() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetVinVoltage()) }
}

#[inline(always)]
pub fn get_vin_current() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetVinCurrent()) }
}

#[inline(always)]
pub fn get_user_voltage6v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage6V()) }
}

#[inline(always)]
pub fn get_user_current6v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent6V()) }
}

#[inline(always)]
pub fn get_user_active6v() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive6V()).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_user_current_faults6v() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults6V()) }
}

#[inline(always)]
pub fn get_user_voltage5v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage5V()) }
}

#[inline(always)]
pub fn get_user_current5v() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent5V()) }
}

#[inline(always)]
pub fn get_user_active5v() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive5V()).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_user_current_faults5v() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults5V()) }
}

#[inline(always)]
pub fn get_user_voltage3v3() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserVoltage3V3()) }
}

#[inline(always)]
pub fn get_user_current3v3() -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetUserCurrent3V3()) }
}

#[inline(always)]
pub fn get_user_active3v3() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetUserActive3V3()).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_user_current_faults3v3() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetUserCurrentFaults3V3()) }
}
