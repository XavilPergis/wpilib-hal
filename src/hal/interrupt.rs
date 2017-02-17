use ::error::*;
use hal::analog_trigger::AnalogTriggerType;
use hal::handle::*;
use ::raw::*;
use std::os::raw::c_void;

pub fn initialize_interrupts(watcher: bool) -> HalResult<InterruptHandle> {
    hal_call![ ptr HAL_InitializeInterrupts(watcher as HAL_Bool) ]
}

pub fn clean_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanInterrupts(handle) ]
}

pub fn wait_for_interrupt(handle: InterruptHandle, timeout: f64, ignore_previous: bool)
                          -> HalResult<i64> {
    hal_call![ ptr HAL_WaitForInterrupt(handle, timeout, ignore_previous as HAL_Bool) ]
}

pub fn enable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_EnableInterrupts(handle) ]
}

pub fn disable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_DisableInterrupts(handle) ]
}

pub fn read_interrupt_rising_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_ReadInterruptRisingTimestamp(handle) ]
}

pub fn read_interrupt_falling_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_ReadInterruptFallingTimestamp(handle) ]
}

pub fn request_interrupts(handle: InterruptHandle, digital_source_handle: DigitalHandle, analog_trigger_type: AnalogTriggerType) -> HalResult<()> {
    hal_call![ ptr HAL_RequestInterrupts(handle, digital_source_handle, analog_trigger_type.into_raw()) ]
}

unsafe extern "C" fn interrupt_handler_cb<F>(interrupt_asserted_mask: u32, closure: *mut c_void) where F: Fn(u32) {
    let opt_closure = closure as *mut Option<F>;
    (*opt_closure).take().unwrap()(interrupt_asserted_mask as u32);
}

/// ## Example
/// ```rust,no_run
/// interrupt::attach_interrupt_handler(5, |mask| {
///     println!("Handler called! {}", mask);
/// })
/// ```
pub fn attach_interrupt_handler<F>(handle: InterruptHandle, handler: F) -> HalResult<()> where F: Fn(u32) {
    let extern_handler = &handler as *const _ as *mut c_void;
    hal_call![ ptr HAL_AttachInterruptHandler(handle, Some(interrupt_handler_cb::<F>), extern_handler) ]
}

// ?TODO
pub fn attach_interrupt_handler_threaded<F>(handle: InterruptHandle, handler: F) -> HalResult<()>
    where F: Fn(u32)
{
    let extern_handler = &handler as *const _ as *mut c_void;
    hal_call![ ptr HAL_AttachInterruptHandlerThreaded(handle, Some(interrupt_handler_cb::<F>), extern_handler) ]
}

pub fn set_interrupt_up_source_edge(handle: InterruptHandle, rising_edge: bool,
                                    falling_edge: bool)
                                    -> HalResult<()> {
    hal_call![ ptr HAL_SetInterruptUpSourceEdge(handle, rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}
