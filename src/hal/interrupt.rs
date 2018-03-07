use std::os::raw::{c_void, c_double};
use hal::analog_trigger::AnalogTriggerType;
use hal::types::{InterruptHandle, Handle, NativeBool};
use error::*;

// Not an Option<T> because we always provide the handler function.
pub type InterruptHandlerFunction = unsafe extern "C" fn(interruptAssertedMask: u32, param: *mut c_void);

extern "C" {
    fn HAL_InitializeInterrupts(watcher: NativeBool, status: *mut i32) -> InterruptHandle;
    fn HAL_CleanInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    fn HAL_WaitForInterrupt(interruptHandle: InterruptHandle, timeout: c_double, ignorePrevious: NativeBool, status: *mut i32) -> i64;
    fn HAL_EnableInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    fn HAL_DisableInterrupts(interruptHandle: InterruptHandle, status: *mut i32);
    // TODO
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

pub struct InterruptHandler {
    pub(crate) handle: Handle
}

impl InterruptHandler {
    pub fn new(watcher: bool) -> HalResult<Self> {
        unsafe {
            hal_call!(HAL_InitializeInterrupts(watcher as NativeBool))
                .map(|handle| InterruptHandler { handle })
        }
    }

    pub fn clean(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_CleanInterrupts(self.handle)) }
    }
/**
 * In synchronous mode, wait for the defined interrupt to occur.
 * @param timeout Timeout in seconds
 * @param ignorePrevious If true, ignore interrupts that happened before
 * waitForInterrupt was called.
 * @return The mask of interrupts that fired.
 */
    pub fn wait(&self, timeout: f64, ignore_previous: bool) -> HalResult<i64> {
        unsafe { hal_call!(HAL_WaitForInterrupt(self.handle, timeout as c_double, ignore_previous as NativeBool)) }
    }

    pub fn enable(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_EnableInterrupts(self.handle)) }
    }

    pub fn disable(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_DisableInterrupts(self.handle)) }
    }

    // TODO: Does F need to be Send or Sync?
    // Static lifetime is required because references onto a stack frame could persist while the
    // stack frame is freed.
    pub fn attach_handler<F: Fn(u32) + 'static>(&self, mut func: F) -> HalResult<()> {
        // Ok so this function might need a little bit of explaining.

        // The interrupt handler register takes a function pointer and a void pointer as a user param.
        // Whenever an interrupt is received, the HAL calls out `handler` function with the user param
        // that we pssed in.
        // All we do here is pass in our closure as a user parameter and call it in the handler.
        #[inline(never)]
        unsafe extern "C" fn handler<F: Fn(u32)>(mask: u32, param: *mut c_void) {
            let func = param as *mut F;
            (*func)(mask);
        }

        unsafe {
            // turn our closure into a void pointer
            let user_param = &mut func as *mut _ as *mut c_void;
            // we need to parameterize `handler` because it cannot use the `F` of the parent scope.
            hal_call!(HAL_AttachInterruptHandler(self.handle, handler::<F>, user_param))
        }
    }
}

impl Drop for InterruptHandler {
    fn drop(&mut self) {
        // AGAIN, this function has a status param that isn't used
        unsafe { HAL_CleanInterrupts(self.handle, ::std::ptr::null_mut()) }
    }
}
