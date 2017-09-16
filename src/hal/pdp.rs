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

pub unsafe fn initialize_pdp(module: i32) -> HalResult<()> {
    hal_call!(ptr HAL_InitializePDP(module))
}

pub unsafe fn check_pdp_channel(channel: i32) -> bool {
    HAL_CheckPDPChannel(channel) != 0
}

pub unsafe fn check_pdp_module(module: i32) -> bool {
    HAL_CheckPDPModule(module) != 0
}

pub unsafe fn get_pdp_temperature(module: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPTemperature(module))
}

pub unsafe fn get_pdp_voltage(module: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPVoltage(module))
}

pub unsafe fn get_pdp_channel_current(module: i32, channel: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPChannelCurrent(module, channel))
}

pub unsafe fn get_pdp_total_current(module: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPTotalCurrent(module))
}

pub unsafe fn get_pdp_total_power(module: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPTotalPower(module))
}

pub unsafe fn get_pdp_total_energy(module: i32) -> HalResult<f64> {
    hal_call!(ptr HAL_GetPDPTotalEnergy(module))
}

pub unsafe fn reset_pdp_total_energy(module: i32) -> HalResult<()> {
    hal_call!(ptr HAL_ResetPDPTotalEnergy(module))
}

pub unsafe fn clear_pdp_sticky_faults(module: i32) -> HalResult<()> {
    hal_call!(ptr HAL_ClearPDPStickyFaults(module))
}
