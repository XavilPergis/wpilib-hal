use ::raw::*;
use hal::handle::*;
use ::error::*;
use std::os::raw::c_void;
use hal::analog_trigger::AnalogTriggerType;

pub fn initialize_interrupts(watcher: bool) -> HalResult<InterruptHandle> {
    hal_call![ ptr HAL_InitializeInterrupts(watcher as HAL_Bool) ].map(|n| InterruptHandle(n))
}

pub fn clean_interrupts(handle: InterruptHandle) -> HalResult<()> {
    hal_call![ ptr HAL_CleanInterrupts(handle.get_handle()) ]
}

pub fn wait_for_interrupt(handle: InterruptHandle,
                          timeout: f64,
                          ignore_previous: bool)
                          -> HalResult<i64> {
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

pub fn request_interrupts<H>(handle: InterruptHandle,
                             digital_source_handle: &H,
                             analog_trigger_type: AnalogTriggerType)
                             -> HalResult<()>
    where H: Handle
{
    hal_call![ ptr HAL_RequestInterrupts(handle.get_handle(), digital_source_handle.get_handle(), analog_trigger_type.into_raw()) ]
}

// ::std::option::Option<unsafe extern "C" fn(interruptAssertedMask: u32, param: *mut c_void)>;
// Fn(u32, Any)

unsafe extern "C" fn interrupt_handler_cb<F>(interrupt_asserted_mask: u32, closure: *mut c_void)
    where F: Fn(u32)
{
    let opt_closure = closure as *mut Option<F>;
    unsafe {
        (*opt_closure).take().unwrap()(interrupt_asserted_mask as u32);
    }


}

// interrupt::attach_interrupt_handler(InterruptHandle(5), |mask| {
//     println!("Handler called! {}", mask);
// })

pub fn attach_interrupt_handler<F>(handle: InterruptHandle, handler: F) -> HalResult<()>
    where F: Fn(u32)
{
    let extern_handler = &handler as *const _ as *mut c_void;
    hal_call![ ptr HAL_AttachInterruptHandler(handle.get_handle(), Some(interrupt_handler_cb::<F>), extern_handler) ]
}

// ?TODO
pub fn attach_interrupt_handler_threaded<F>(handle: InterruptHandle, handler: F) -> HalResult<()>
    where F: Fn(u32)
{
    let extern_handler = &handler as *const _ as *mut c_void;
    hal_call![ ptr HAL_AttachInterruptHandlerThreaded(handle.get_handle(), Some(interrupt_handler_cb::<F>), extern_handler) ]
}

pub fn set_interrupt_up_source_edge(handle: InterruptHandle,
                                    rising_edge: bool,
                                    falling_edge: bool)
                                    -> HalResult<()> {
    hal_call![ ptr HAL_SetInterruptUpSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
}
