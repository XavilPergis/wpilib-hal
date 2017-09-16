use ::error::*;
use handle::*;
use ::raw::*;

pub unsafe fn initialize_port(handle: PortHandle) -> HalResult<SolenoidHandle> {
    hal_call!(ptr HAL_InitializeSolenoidPort(handle))
}

pub unsafe fn free_port(handle: SolenoidHandle) {
    HAL_FreeSolenoidPort(handle)
}

pub unsafe fn check_module(module: i32) -> bool {
    HAL_CheckSolenoidModule(module) != 0
}

pub unsafe fn check_channel(channel: i32) -> bool {
    HAL_CheckSolenoidChannel(channel) != 0
}

pub unsafe fn get(handle: SolenoidHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetSolenoid(handle)).map(|n| n != 0)
}

pub unsafe fn get_all_solenoids(module: i32) -> HalResult<i32> {
    hal_call!(ptr HAL_GetAllSolenoids(module))
}

pub unsafe fn set(solenoid_port_handle: SolenoidHandle, value: bool) -> HalResult<()> {
    hal_call!(ptr HAL_SetSolenoid(solenoid_port_handle, value as HAL_Bool))
}

pub unsafe fn set_all_solenoids(module: i32, state: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetAllSolenoids(module, state))
}

pub unsafe fn get_pcm_black_list(module: i32) -> HalResult<i32> {
    hal_call!(ptr HAL_GetPCMSolenoidBlackList(module))
}

pub unsafe fn get_pcm_voltage_sticky_fault(module: i32) -> HalResult<bool> {
    hal_call!(ptr HAL_GetPCMSolenoidVoltageStickyFault(module)).map(|n| n != 0)
}

pub unsafe fn get_pcm_voltage_fault(module: i32) -> HalResult<bool> {
    hal_call!(ptr HAL_GetPCMSolenoidVoltageFault(module)).map(|n| n != 0)
}

pub unsafe fn clear_all_pcm_sticky_faults(module: i32) -> HalResult<()> {
    hal_call!(ptr HAL_ClearAllPCMStickyFaults(module))
}
