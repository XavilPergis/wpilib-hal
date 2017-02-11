use ::error::*;
use ::raw::*;

pub fn initialize_pdp(module: i32) -> HalResult<()> {
    hal_call![ ptr HAL_InitializePDP(module) ]
}

pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPDPChannel(channel) != 0 }
}

pub fn check_pdp_module(module: i32) -> bool {
    unsafe { HAL_CheckPDPModule(module) != 0 }
}

pub fn get_pdp_temperature(module: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPTemperature(module) ]
}

pub fn get_pdp_voltage(module: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPVoltage(module) ]
}

pub fn get_pdp_channel_current(module: i32, channel: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPChannelCurrent(module, channel) ]
}

pub fn get_pdp_total_current(module: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPTotalCurrent(module) ]
}

pub fn get_pdp_total_power(module: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPTotalPower(module) ]
}

pub fn get_pdp_total_energy(module: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPDPTotalEnergy(module) ]
}

pub fn reset_pdp_total_energy(module: i32) -> HalResult<()> {
    hal_call![ ptr HAL_ResetPDPTotalEnergy(module) ]
}

pub fn clear_pdp_sticky_faults(module: i32) -> HalResult<()> {
    hal_call![ ptr HAL_ClearPDPStickyFaults(module) ]
}
