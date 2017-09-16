use ::error::*;
use hal::handle::*;
use ::raw::*;

pub struct CompressorFaults {
    pub current_too_high: bool,
    pub current_too_high_sticky: bool,
    pub shorted: bool,
    pub shorted_sticky: bool,
    pub not_connected: bool,
    pub not_connected_sticky: bool,
}

pub fn initialize(module: i32) -> HalResult<CompressorHandle> {
    unsafe { hal_call!(ptr HAL_InitializeCompressor(module)) }
}

pub fn check_module(module: i32) -> bool {
    unsafe { HAL_CheckCompressorModule(module) != 0 }
}

pub fn get(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressor(handle)).map(|n| n != 0) }
}

pub fn set_closed_loop_control(handle: CompressorHandle, value: bool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetCompressorClosedLoopControl(handle, value as HAL_Bool)) }
}

pub fn get_closed_loop_control(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorClosedLoopControl(handle)).map(|n| n != 0) }
}

pub fn get_pressure_switch(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorPressureSwitch(handle)).map(|n| n != 0) }
}

pub fn get_current(handle: CompressorHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrent(handle)) }
}

pub fn get_faults(handle: CompressorHandle) -> HalResult<CompressorFaults> {
    Ok(CompressorFaults {
        current_too_high:        get_current_too_high_fault(handle)?,
        current_too_high_sticky: get_current_too_high_sticky_fault(handle)?,
        shorted:                 get_shorted_fault(handle)?,
        shorted_sticky:          get_shorted_sticky_fault(handle)?,
        not_connected:           get_not_connected_fault(handle)?,
        not_connected_sticky:    get_not_connected_sticky_fault(handle)?,
    })
}

pub fn get_current_too_high_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighFault(handle)).map(|n| n != 0) }
}

pub fn get_current_too_high_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighStickyFault(handle)).map(|n| n != 0) }
}

pub fn get_shorted_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorShortedStickyFault(handle)).map(|n| n != 0) }
}

pub fn get_shorted_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorShortedFault(handle)).map(|n| n != 0) }
}

pub fn get_not_connected_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedStickyFault(handle)).map(|n| n != 0) }
}

pub fn get_not_connected_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedFault(handle)).map(|n| n != 0) }
}
