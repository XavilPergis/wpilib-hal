use ::raw::*;

pub fn initialize_compressor(module: i32) -> HalResult<CompressorHandle> {
    hal_call![ ptr HAL_InitializeCompressor(module) ]
}

pub fn check_compressor_module(module: i32) -> bool {
    unsafe { HAL_CheckCompressorModule(module) != 0 }
}

pub fn get_compressor(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressor(handle.get_handle()) ].map(|n| n != 0)
}

pub fn set_compressor_closed_loop_control(handle: CompressorHandle, value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetCompressorClosedLoopControl(handle.get_handle(), value as HAL_Bool) ]
}

pub fn get_compressor_closed_loop_control(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorClosedLoopControl(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_pressure_switch(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorPressureSwitch(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_current(handle: CompressorHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetCompressorCurrent(handle.get_handle()) ]
}

pub fn get_compressor_current_too_high_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorCurrentTooHighFault(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_current_too_high_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorCurrentTooHighStickyFault(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_shorted_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorShortedStickyFault(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_shorted_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorShortedFault(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_not_connected_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorNotConnectedStickyFault(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_compressor_not_connected_fault(handle: CompressorHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetCompressorNotConnectedFault(handle.get_handle()) ].map(|n| n != 0)
}
