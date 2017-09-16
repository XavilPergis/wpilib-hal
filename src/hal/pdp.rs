use error::*;
use hal::types::NativeBool;
use std::os::raw::c_double;

extern "C" {
    fn HAL_InitializePDP(module: i32, status: *mut i32);
    fn HAL_CheckPDPChannel(channel: i32) -> NativeBool;
    fn HAL_CheckPDPModule(module: i32) -> NativeBool;
    fn HAL_GetPDPTemperature(module: i32, status: *mut i32) -> c_double;
    fn HAL_GetPDPVoltage(module: i32, status: *mut i32) -> c_double;
    fn HAL_GetPDPChannelCurrent(module: i32, channel: i32, status: *mut i32) -> c_double;
    fn HAL_GetPDPTotalCurrent(module: i32, status: *mut i32) -> c_double;
    fn HAL_GetPDPTotalPower(module: i32, status: *mut i32) -> c_double;
    fn HAL_GetPDPTotalEnergy(module: i32, status: *mut i32) -> c_double;
    fn HAL_ResetPDPTotalEnergy(module: i32, status: *mut i32);
    fn HAL_ClearPDPStickyFaults(module: i32, status: *mut i32);
}

#[inline(always)]
pub fn initialize_pdp(module: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_InitializePDP(module)) }
}

#[inline(always)]
pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPDPChannel(channel) != 0 }
}

#[inline(always)]
pub fn check_pdp_module(module: i32) -> bool {
    unsafe { HAL_CheckPDPModule(module) != 0 }
}

#[inline(always)]
pub fn get_pdp_temperature(module: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPTemperature(module)) }
}

#[inline(always)]
pub fn get_pdp_voltage(module: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPVoltage(module)) }
}

#[inline(always)]
pub fn get_pdp_channel_current(module: i32, channel: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPChannelCurrent(module, channel)) }
}

#[inline(always)]
pub fn get_pdp_total_current(module: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPTotalCurrent(module)) }
}

#[inline(always)]
pub fn get_pdp_total_power(module: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPTotalPower(module)) }
}

#[inline(always)]
pub fn get_pdp_total_energy(module: i32) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetPDPTotalEnergy(module)) }
}

#[inline(always)]
pub fn reset_pdp_total_energy(module: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ResetPDPTotalEnergy(module)) }
}

#[inline(always)]
pub fn clear_pdp_sticky_faults(module: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ClearPDPStickyFaults(module)) }
}
