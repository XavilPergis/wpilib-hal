use error::*;
use hal::types::{NativeBool, CompressorHandle};

extern "C" {
    fn HAL_InitializeCompressor(module: i32, status: *mut i32) -> CompressorHandle;
    fn HAL_GetCompressor(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetCompressorClosedLoopControl(handle: CompressorHandle, value: NativeBool, status: *mut i32);
    fn HAL_GetCompressorClosedLoopControl(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorPressureSwitch(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorCurrent(handle: CompressorHandle, status: *mut i32) -> ::std::os::raw::c_double;
    fn HAL_GetCompressorCurrentTooHighStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorCurrentTooHighFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorShortedStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorShortedFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorNotConnectedStickyFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetCompressorNotConnectedFault(handle: CompressorHandle, status: *mut i32) -> NativeBool;
}

#[derive(Debug)]
struct Compressor {
    module: i32,
    handle: i32,
}

impl Compressor {
    pub fn new() -> HalResult<Self> {
        // TODO
        Self::initialize(0)
    }

    pub fn initialize(module: i32) -> HalResult<Self> {
        let handle = unsafe { hal_call!(ptr HAL_InitializeCompressor(module))? };
        
        Ok(Compressor { module, handle })
    }

    pub fn enabled(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressor(self.handle)).map(|n| n != 0) }
    }

    pub fn pressure_switch_low(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorPressureSwitch(self.handle)).map(|n| n != 0) }
    }

    pub fn set_closed_loop_control(&self, closed_loop: bool) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetCompressorClosedLoopControl(self.handle, closed_loop as NativeBool)) }
    }

    pub fn get_closed_loop_control(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorClosedLoopControl(self.handle)).map(|n| n != 0) }
    }

    pub fn get_current(&self) -> HalResult<f64> {
        unsafe { hal_call!(ptr HAL_GetCompressorCurrent(self.handle)).map(|a| a as f64) }
    }
    
    pub fn get_current_too_high_sticky_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighStickyFault(self.handle)).map(|a| a != 0) }
    }
    
    pub fn get_current_too_high_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorCurrentTooHighFault(self.handle)).map(|a| a != 0) }
    }
    
    pub fn get_shorted_sticky_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorShortedStickyFault(self.handle)).map(|a| a != 0) }
    }
    
    pub fn get_shorted_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorShortedFault(self.handle)).map(|a| a != 0) }
    }
    
    pub fn get_not_connected_sticky_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedStickyFault(self.handle)).map(|a| a != 0) }
    }
    
    pub fn get_not_connected_fault(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetCompressorNotConnectedFault(self.handle)).map(|a| a != 0) }
    }
}
