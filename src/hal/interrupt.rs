use ::raw::*;

pub fn initialize_interrupts(watcher: bool) -> HalResult<InterruptHandle> {
    hal_call![ ptr HAL_InitializeInterrupts(watcher as HAL_Bool) ]
}

pub fn clean_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanInterrupts(handle.get_handle()) ]
}

pub fn wait_for_interrupt(handle: InterruptHandle, timeout: f64, ignore_previous: bool) -> HalResult<i64> {
    hal_call![ ptr HAL_WaitForInterrupt(handle.get_handle(), timeout, ignore_previous as HAL_Bool) ]
}

pub fn enable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_EnableInterrupts(handle.get_handle()) ]
}

pub fn disable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_DisableInterrupts(handle.get_handle()) ]
}

pub fn read_interrupt_rising_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_ReadInterruptRisingTimestamp(handle.get_handle()) ]
}

pub fn read_interrupt_falling_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_ReadInterruptFallingTimestamp(handle.get_handle()) ]
}

pub fn request_interrupts<H>(handle: InterruptHandle, digital_source_handle: &H, analog_trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
    hal_call![ ptr HAL_RequestInterrupts(handle.get_handle(), digital_source_handle.get_handle(), analog_trigger_type.into()) ]
}

// ?TODO
pub fn attach_interrupt_handler(handle: InterruptHandle, handler: HAL_InterruptHandlerFunction, param: *mut ::std::os::raw::c_void) -> HalResult<()> {
    hal_call![ ptr HAL_AttachInterruptHandler(handle.get_handle()) ]
}

// ?TODO
pub fn attach_interrupt_handler_threaded(handle: InterruptHandle, handler: HAL_InterruptHandlerFunction, param: *mut ::std::os::raw::c_void) -> HalResult<()> {
    hal_call![ ptr HAL_AttachInterruptHandlerThreaded(handle.get_handle()) ]
}

pub fn set_interrupt_up_source_edge(handle: InterruptHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetInterruptUpSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}
