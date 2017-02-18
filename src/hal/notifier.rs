use ::error::*;
use hal::handle::*;
use ::raw::*;

pub type HAL_NotifierProcessFunction = ::std::option::Option<unsafe extern "C" fn(currentTime: u64, handle: HAL_NotifierHandle)>;

/*
unsafe extern "C" fn interrupt_handler_cb<F>(interrupt_asserted_mask: u32, closure: *mut c_void) where F: Fn(u32) {
    let opt_closure = closure as *mut Option<F>;
    (*opt_closure).take().unwrap()(interrupt_asserted_mask as u32);
}

pub fn attach_interrupt_handler<F>(handle: InterruptHandle, handler: F) -> HalResult<()> where F: Fn(u32) {
    let extern_handler = &handler as *const _ as *mut c_void;
    unsafe { hal_call![ ptr HAL_AttachInterruptHandler(handle, Some(interrupt_handler_cb::<F>), extern_handler) ] }
}

pub fn attach_interrupt_handler_threaded<F>(handle: InterruptHandle, handler: F) -> HalResult<()>
    where F: Fn(u32)
{
    let extern_handler = &handler as *const _ as *mut c_void;
    unsafe { hal_call![ ptr HAL_AttachInterruptHandlerThreaded(handle, Some(interrupt_handler_cb::<F>), extern_handler) ] }
}

pub fn HAL_InitializeNotifier(process: HAL_NotifierProcessFunction,
                              param: *mut ::std::os::raw::c_void,
                              status: *mut i32) -> HAL_NotifierHandle;
*/


// unsafe extern "C" fn generate_callback<F>(func: F) -> HAL_NotifierProcessFunction where F: Fn(u64, HAL_NotifierHandle) {
//
//     unsafe extern "C" fn callback(time: u64, handle: NotifierHandle) {
//         func(time, handle)
//     }
//
//     Some(callback)
// }
//
// pub fn initialize_notifier<F>(handler: F) -> HalResult<NotifierHandle> where F: Fn(u64, NotifierHandle) {
//     let extern_handler = &handler as *const _ as *mut ::std::os::raw::c_void;
//     let callback = generate_callback::<F>(handler);
//     unsafe { hal_call![ ptr HAL_InitializeNotifier(callback, extern_handler) ] }
// }
//
// pub fn initialize_notifier_threaded(process: HAL_NotifierProcessFunction,
//                                     param: *mut ::std::os::raw::c_void)
//                                     -> HalResult<NotifierHandle> {
//     unsafe { hal_call![ ptr HAL_InitializeNotifierThreaded() ] }
// }

pub fn clean_notifier(handle: NotifierHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_CleanNotifier(handle) ] }
}

// Oh fuck
pub fn get_notifier_param(handle: NotifierHandle) -> HalResult<*mut ::std::os::raw::c_void> {
    unsafe { hal_call![ ptr HAL_GetNotifierParam(handle) ] }
}

pub fn update_notifier_alarm(handle: NotifierHandle, trigger_time: u64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_UpdateNotifierAlarm(handle, trigger_time) ] }
}

pub fn stop_notifier_alarm(handle: NotifierHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_StopNotifierAlarm(handle) ] }
}
