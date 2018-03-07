use error::*;
use hal::types::*;
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

fn check_channel(channel: i32) -> bool { unsafe { HAL_CheckPDPChannel(channel) != 0 } }
fn check_module(channel: i32) -> bool { unsafe { HAL_CheckPDPModule(channel) != 0 } }

pub struct PowerDistributionPanel {
    pub(crate) module: i32,
}

impl PowerDistributionPanel {
    pub fn new(module: i32) -> HalResult<Self> {
        if !check_module(module) { return Err(HalError::InvalidModule(module)); }
        unsafe { hal_call!(HAL_InitializePDP(module))?; }
        Ok(PowerDistributionPanel { module })
    }

    pub fn get_voltage(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetPDPVoltage(self.module)) }
    }

    pub fn get_temperature(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetPDPTemperature(self.module)) }
    }

    pub fn get_current(&self, channel: i32) -> HalResult<f64> {
        if !check_channel(channel) { return Err(HalError::InvalidChannel(channel)); }
        unsafe { hal_call!(HAL_GetPDPChannelCurrent(self.module, channel)) }
    }
    
    pub fn get_total_current(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetPDPTotalCurrent(self.module)) }
    }
    
    pub fn get_total_power(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetPDPTotalPower(self.module)) }
    }
    
    pub fn get_total_energy(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetPDPTotalEnergy(self.module)) }
    }
    
    pub fn reset_total_energy(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_ResetPDPTotalEnergy(self.module)) }
    }
    
    pub fn clear_sticky_faults(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_ClearPDPStickyFaults(self.module)) }
    }
}
