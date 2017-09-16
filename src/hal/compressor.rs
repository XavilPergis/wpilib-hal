use error::*;
use hal::types::{NativeBool, CompressorHandle};

extern "C" {
    fn HAL_InitializeCompressor(module: i32, status: *mut i32) -> CompressorHandle;
    fn HAL_CheckCompressorModule(module: i32) -> NativeBool;
    fn HAL_GetCompressor(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetCompressorClosedLoopControl(handle: CompressorHandle, value: NativeBool, status: *mut i32);
    fn HAL_GetCompressorClosedLoopControl(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorPressureSwitch(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorCurrent(handle: CompressorHandle, status: *mut i32) -> ::std::os::raw::c_double;
    fn HAL_GetCompressorCurrentTooHighFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorCurrentTooHighStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorShortedStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorShortedFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorNotConnectedStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorNotConnectedFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
}

pub struct CompressorFaults {
    pub current_too_high: bool,
    pub current_too_high_sticky: bool,
    pub shorted: bool,
    pub shorted_sticky: bool,
    pub not_connected: bool,
    pub not_connected_sticky: bool,
}

#[inline(always)]
pub fn initialize(module: i32) -> HalResult<CompressorHandle> {
    unsafe { hal_call!(ptr HAL_InitializeCompressor(module)) }
}

#[inline(always)]
pub fn check_module(module: i32) -> bool {
    unsafe { HAL_CheckCompressorModule(module) != 0 }
}

#[inline(always)]
pub fn get(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressor(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn set_closed_loop_control(handle: CompressorHandle, value: bool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetCompressorClosedLoopControl(handle, value as NativeBool)) }
}

#[inline(always)]
pub fn get_closed_loop_control(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorClosedLoopControl(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_pressure_switch(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorPressureSwitch(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_current(handle: CompressorHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrent(handle)) }
}

pub fn get_all_faults(handle: CompressorHandle) -> HalResult<CompressorFaults> {
    Ok(CompressorFaults {
        current_too_high:        get_current_too_high_fault(handle)?,
        current_too_high_sticky: get_current_too_high_sticky_fault(handle)?,
        shorted:                 get_shorted_fault(handle)?,
        shorted_sticky:          get_shorted_sticky_fault(handle)?,
        not_connected:           get_not_connected_fault(handle)?,
        not_connected_sticky:    get_not_connected_sticky_fault(handle)?,
    })
}

#[inline(always)]
pub fn get_current_too_high_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighFault(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_current_too_high_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighStickyFault(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_shorted_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorShortedStickyFault(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_shorted_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorShortedFault(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_not_connected_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedStickyFault(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_not_connected_fault(handle: CompressorHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedFault(handle)).map(|n| n != 0) }
}
