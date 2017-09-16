use error::*;
use hal::types::{PortHandle, SolenoidHandle, NativeBool};

extern "C" {
    fn HAL_InitializeSolenoidPort(handle: PortHandle, status: *mut i32) -> SolenoidHandle;
    fn HAL_FreeSolenoidPort(handle: SolenoidHandle);
    fn HAL_CheckSolenoidModule(module: i32) -> NativeBool;
    fn HAL_CheckSolenoidChannel(channel: i32) -> NativeBool;
    fn HAL_GetSolenoid(handle: SolenoidHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetAllSolenoids(module: i32, status: *mut i32) -> i32;
    fn HAL_SetSolenoid(handle: SolenoidHandle, value: NativeBool, status: *mut i32);
    fn HAL_SetAllSolenoids(module: i32, state: i32, status: *mut i32);
    fn HAL_GetPCMSolenoidBlackList(module: i32, status: *mut i32) -> i32;
    fn HAL_GetPCMSolenoidVoltageStickyFault(module: i32, status: *mut i32) -> NativeBool;
    fn HAL_GetPCMSolenoidVoltageFault(module: i32, status: *mut i32) -> NativeBool;
    fn HAL_ClearAllPCMStickyFaults(module: i32, status: *mut i32);
}

#[inline(always)]
pub fn initialize_port(handle: PortHandle) -> HalResult<SolenoidHandle> {
    unsafe { hal_call!(ptr HAL_InitializeSolenoidPort(handle)) }
}

#[inline(always)]
pub fn free_port(handle: SolenoidHandle) {
    unsafe { HAL_FreeSolenoidPort(handle) }
}

#[inline(always)]
pub fn check_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

#[inline(always)]
pub fn check_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

#[inline(always)]
pub fn get(handle: SolenoidHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetSolenoid(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_all_solenoids(module: i32) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetAllSolenoids(module)) }
}

#[inline(always)]
pub fn set(solenoid_port_handle: SolenoidHandle, value: bool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetSolenoid(solenoid_port_handle, value as NativeBool)) }
}

#[inline(always)]
pub fn set_all_solenoids(module: i32, state: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetAllSolenoids(module, state)) }
}

#[inline(always)]
pub fn get_pcm_black_list(module: i32) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetPCMSolenoidBlackList(module)) }
}

#[inline(always)]
pub fn get_pcm_voltage_sticky_fault(module: i32) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetPCMSolenoidVoltageStickyFault(module)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_pcm_voltage_fault(module: i32) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetPCMSolenoidVoltageFault(module)).map(|n| n != 0) }
}

#[inline(always)]
pub fn clear_all_pcm_sticky_faults(module: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ClearAllPCMStickyFaults(module)) }
}
