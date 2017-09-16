use std::os::raw::{c_void, c_double};
use hal::analog::AnalogTriggerType;
use hal::types::{InterruptHandle, Handle, DigitalHandle, NativeBool};
use error::*;

// TODO: can this function pointer actually be nullable?
pub type InterruptHandlerFunction = Option<unsafe extern "C" fn(interruptAssertedMask: u32, param: *mut c_void)>;

extern "C" {
    fn HAL_InitializeInterrupts(watcher: NativeBool, status: *mut i32) -> InterruptHandle;
    fn HAL_CleanInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    fn HAL_WaitForInterrupt(interruptHandle: InterruptHandle, timeout: c_double, ignorePrevious: NativeBool, status: *mut i32) -> i64;
    fn HAL_EnableInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    fn HAL_DisableInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    fn HAL_ReadInterruptRisingTimestamp(interruptHandle: InterruptHandle, status: *mut i32) -> c_double;
    fn HAL_ReadInterruptFallingTimestamp(interruptHandle: InterruptHandle, status: *mut i32) -> c_double;
    fn HAL_RequestInterrupts(interruptHandle: InterruptHandle,
                                 digitalSourceHandle: Handle,
                                 analogTriggerType: AnalogTriggerType,
                                 status: *mut i32);
    fn HAL_AttachInterruptHandler(interruptHandle: InterruptHandle,
                                      handler: InterruptHandlerFunction,
                                      param: *mut c_void,
                                      status: *mut i32);
    fn HAL_AttachInterruptHandlerThreaded(interruptHandle: InterruptHandle,
                                              handler: InterruptHandlerFunction,
                                              param: *mut c_void,
                                              status: *mut i32);
    fn HAL_SetInterruptUpSourceEdge(interruptHandle: InterruptHandle,
                                        risingEdge: NativeBool,
                                        fallingEdge: NativeBool,
                                        status: *mut i32);
}

pub unsafe fn initialize_interrupts(watcher: bool) -> HalResult<InterruptHandle> {
    hal_call!(ptr HAL_InitializeInterrupts(watcher as NativeBool))
}

pub unsafe fn clean_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call!(ptr HAL_CleanInterrupts(handle))
}

pub unsafe fn wait_for_interrupt(handle: InterruptHandle, timeout: f64, ignore_previous: bool) -> HalResult<i64> {
    hal_call!(ptr HAL_WaitForInterrupt(handle, timeout, ignore_previous as NativeBool))
}

pub unsafe fn enable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call!(ptr HAL_EnableInterrupts(handle))
}

pub unsafe fn disable_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call!(ptr HAL_DisableInterrupts(handle))
}

pub unsafe fn read_interrupt_rising_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_ReadInterruptRisingTimestamp(handle))
}

pub unsafe fn read_interrupt_falling_timestamp(handle: InterruptHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_ReadInterruptFallingTimestamp(handle))
}

pub unsafe fn request_interrupts(handle: InterruptHandle, digital_source_handle: DigitalHandle, analog_trigger_type: AnalogTriggerType) -> HalResult<()> {
    hal_call!(ptr HAL_RequestInterrupts(handle, digital_source_handle, analog_trigger_type))
}

unsafe extern "C" fn interrupt_handler_cb<F>(interrupt_asserted_mask: u32, closure: *mut c_void) where F: Fn(u32) {
    let opt_closure = closure as *mut Option<F>;
    (*opt_closure).take().unwrap()(interrupt_asserted_mask as u32);
}

/// Attach a handler that is activated when the handle recieves an interrupt
///
/// ## Example
/// ```rust,no_run
/// interrupt::attach_interrupt_handler(5, |mask| {
///     println!("Handler called! {}", mask);
/// })
/// ```
pub unsafe fn attach_interrupt_handler<F>(handle: InterruptHandle, mut handler: F) -> HalResult<()> where F: Fn(u32) {
    let extern_handler = &mut handler as *mut _ as *mut c_void;
    hal_call!(ptr HAL_AttachInterruptHandler(handle, Some(interrupt_handler_cb::<F>), extern_handler))
}

pub unsafe fn attach_interrupt_handler_threaded<F>(handle: InterruptHandle, mut handler: F) -> HalResult<()> where F: Fn(u32) {
    let extern_handler = &mut handler as *mut _ as *mut c_void;
    hal_call!(ptr HAL_AttachInterruptHandlerThreaded(handle, Some(interrupt_handler_cb::<F>), extern_handler))
}

pub unsafe fn set_interrupt_up_source_edge(handle: InterruptHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
    hal_call!(ptr HAL_SetInterruptUpSourceEdge(handle, rising_edge as NativeBool, falling_edge as NativeBool))
}
