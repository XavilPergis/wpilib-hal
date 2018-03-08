use error::*;
use hal::types::*;
use std::thread::{JoinHandle, spawn};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

extern "C" {
    fn HAL_InitializeNotifier(status: *mut i32) -> NotifierHandle;
    fn HAL_StopNotifier(handle: NotifierHandle, status: *mut i32);
    fn HAL_CleanNotifier(handle: NotifierHandle, status: *mut i32);
    fn HAL_UpdateNotifierAlarm(handle: NotifierHandle, triggerTime: u64, status: *mut i32);
    fn HAL_CancelNotifierAlarm(handle: NotifierHandle, status: *mut i32);
    fn HAL_WaitForNotifierAlarm(handle: NotifierHandle, status: *mut i32);
}

#[derive(Debug)]
pub struct Notifier {
    pub(crate) handle: Handle,
    thread_running: Arc<AtomicBool>,
    period: Arc<AtomicUsize>,
    notifier_thread: Option<JoinHandle<()>>,
}

impl Notifier {
    pub fn new<F: Fn() + Send + 'static>(handler: F, period: usize) -> HalResult<Self> {
        // Initialize a notifier
        let handle = unsafe { hal_call!(HAL_InitializeNotifier())? };
        let period = Arc::new(AtomicUsize::new(period));
        let thread_running = Arc::new(AtomicBool::new(true));

        // Start the listener thread
        let thread_condition = thread_running.clone();
        let thread_period = period.clone();
        let notifier_thread = Some(spawn(move || {
            let mut timeout = ::hal::get_fpga_time().expect("Could not read FPGA time.");

            while thread_condition.load(Ordering::SeqCst) {
                timeout += thread_period.load(Ordering::SeqCst) as u64;

                unsafe {
                    // Update the timer and then wait for it. Unwrap because we're kinda screwed anyways if this fails
                    hal_call!(HAL_UpdateNotifierAlarm(handle, timeout)).expect("Updating notifier failed.");
                    hal_call!(HAL_WaitForNotifierAlarm(handle)).expect("Waiting for notifier alarm failed.");
                }

                handler();
            }
        }));

        Ok(Notifier { handle, notifier_thread, thread_running, period })
    }

    pub fn set_period(&self, period: usize) {
        self.period.store(period, Ordering::SeqCst);
    }

    pub fn cancel_alarm(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_CancelNotifierAlarm(self.handle)) }
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        // Neither stop nor clean set the status param.
        // XXX: probably going to be changed in 2019
        unsafe { HAL_StopNotifier(self.handle, ::std::ptr::null_mut()); }
        self.thread_running.store(false, Ordering::SeqCst);
        // `self.notifier_thread` is only ever `None` after this call. `self.notifier_thread` is
        // only `Option` because of this call. Since joining takes ownership of the JoinHandle
        // and we are provided a `&mut self`, trying to join normally would make the borrow
        // checker complain.
        // We discard any error here because the only thing we care about is disallowing calls
        // to `HAL_WaitForNotifierAlarm` after the notifier has been cleaned up, and a panicked
        // thread means it's halted.
        let _ = self.notifier_thread.take().unwrap().join();
        unsafe { HAL_CleanNotifier(self.handle, ::std::ptr::null_mut()) }
    }
}
