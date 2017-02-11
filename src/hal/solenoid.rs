use ::error::*;

use handle::{Handle, PortHandle, SolenoidHandle};
use ::raw::*;

pub fn initialize_solenoid_port(handle: PortHandle) -> HalResult<SolenoidHandle> {
    hal_call![ ptr HAL_InitializeSolenoidPort(handle.get_handle()) ].map(|n| SolenoidHandle(n))
}

pub fn free_solenoid_port(handle: SolenoidHandle) {
    unsafe { HAL_FreeSolenoidPort(handle.get_handle()) }
}

pub fn check_solenoid_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

pub fn get_solenoid(handle: SolenoidHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetSolenoid(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_all_solenoids(module: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_GetAllSolenoids(module) ]
}

pub fn set_solenoid(solenoid_port_handle: SolenoidHandle, value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetSolenoid(solenoid_port_handle.get_handle(), value as HAL_Bool) ]
}

pub fn set_all_solenoids(module: i32, state: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetAllSolenoids(module, state) ]
}

pub fn get_pcm_solenoid_black_list(module: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_GetPCMSolenoidBlackList(module) ]
}

pub fn get_pcm_solenoid_voltage_sticky_fault(module: i32) -> HalResult<bool> {
    hal_call![ ptr HAL_GetPCMSolenoidVoltageStickyFault(module) ].map(|n| n != 0)
}

pub fn get_pcm_solenoid_voltage_fault(module: i32) -> HalResult<bool> {
    hal_call![ ptr HAL_GetPCMSolenoidVoltageFault(module) ].map(|n| n != 0)
}

pub fn clear_all_pcm_sticky_faults(module: i32) -> HalResult<()> {
    hal_call![ ptr HAL_ClearAllPCMStickyFaults(module) ]
}
