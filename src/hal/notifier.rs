// use ::error::*;
// use hal::handle::*;
// use ::raw::*;
//
// unsafe extern "C" fn foo<F>(current_time: u64, handle: NotifierHandle) where F: Fn(u64, NotifierHandle) {
//     let closure = get_notifier_param(handle);
// }
//
// pub fn initialize_notifier<F>(handler: F) -> HalResult<NotifierHandle> where F: Fn(u64, NotifierHandle) {
//     let extern_handler = &handler as *const _ as *mut ::std::os::raw::c_void;
//     // let callback = generate_callback::<F>(handler);
//     unsafe { hal_call![ ptr HAL_InitializeNotifier(foo, extern_handler) ] }
// }
//
// pub fn initialize_notifier_threaded(process: HAL_NotifierProcessFunction,
//                                     param: *mut ::std::os::raw::c_void)
//                                     -> HalResult<NotifierHandle> {
//     unsafe { hal_call![ ptr HAL_InitializeNotifierThreaded() ] }
// }
//
// pub fn clean_notifier(handle: NotifierHandle) -> HalResult<()> {
//     unsafe { hal_call![ ptr HAL_CleanNotifier(handle) ] }
// }
//
// // Oh fuck
// pub fn get_notifier_param(handle: NotifierHandle) -> HalResult<*mut ::std::os::raw::c_void> {
//     unsafe { hal_call![ ptr HAL_GetNotifierParam(handle) ] }
// }
//
// pub fn update_notifier_alarm(handle: NotifierHandle, trigger_time: u64) -> HalResult<()> {
//     unsafe { hal_call![ ptr HAL_UpdateNotifierAlarm(handle, trigger_time) ] }
// }
//
// pub fn stop_notifier_alarm(handle: NotifierHandle) -> HalResult<()> {
//     unsafe { hal_call![ ptr HAL_StopNotifierAlarm(handle) ] }
// }
