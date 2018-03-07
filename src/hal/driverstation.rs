use std::os::raw::*;
use std::ffi::CString;
use hal::types::*;
use error::*;

extern "C" {
    fn HAL_SendError(is_error: NativeBool, error_code: i32, is_lv_code: NativeBool,
                     details: *const c_char, 
                     location: *const c_char, 
                     call_stack: *const c_char,
                     print_msg: NativeBool) -> i32;
    fn HAL_GetControlWord(control_word: *mut u32) -> i32;
    fn HAL_GetAllianceStation(status: *mut i32) -> AllianceStationID;
    fn HAL_GetMatchTime(status: *mut i32) -> c_double;
    fn HAL_GetMatchInfo(info: *mut MatchInfo) -> c_int;
    fn HAL_FreeMatchInfo(info: *mut MatchInfo);

    fn HAL_ReleaseDSMutex();
    fn HAL_InitializeDriverStation();
    fn HAL_WaitForDSData();
    fn HAL_WaitForDSDataTimeout(timeout: c_double) -> NativeBool;
    fn HAL_IsNewControlData() -> NativeBool;
    fn HAL_ObserveUserProgramStarting();
    fn HAL_ObserveUserProgramDisabled();
    fn HAL_ObserveUserProgramAutonomous();
    fn HAL_ObserveUserProgramTeleop();
    fn HAL_ObserveUserProgramTest();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ControlWord(u32);

impl ControlWord {
    pub fn enabled(&self) -> bool { self.0 & 1 != 0 }
    pub fn autonomous(&self) -> bool { self.0 >> 1 & 1 != 0 }
    pub fn test(&self) -> bool { self.0 >> 2 & 1 != 0 }
    pub fn emergency_stop(&self) -> bool { self.0 >> 3 & 1 != 0 }
    pub fn fms_attached(&self) -> bool { self.0 >> 4 & 1 != 0 }
    pub fn ds_attached(&self) -> bool { self.0 >> 5 & 1 != 0 }
}

pub fn get_control_word() -> HalResult<ControlWord> {
    unsafe {
        let mut word = 0;
        hal_call!(ret HAL_GetControlWord(&mut word))?;
        Ok(ControlWord(word))
    }
}

#[repr(i32)]
pub enum AllianceStationID {
    Red1, Red2, Red3, Blue1, Blue2, Blue3,
}

#[repr(C)]
pub enum MatchType {
    None,
    Practice,
    Qualification,
    Elimination,
}

#[repr(C)]
pub struct MatchInfo {
    event_name: *mut c_char,
    match_type: MatchType,
    match_number: u16,
    replay_number: u8,
    game_specific_message: *mut c_char,
}

impl Drop for MatchInfo {
    fn drop(&mut self) {
        unsafe { HAL_FreeMatchInfo(self); }
    }
}

pub fn get_match_info() -> HalResult<MatchInfo> {
    unsafe {
        let mut match_info = ::std::mem::uninitialized();
        hal_call!(ret HAL_GetMatchInfo(&mut match_info))?;
        Ok(match_info)
    }
}

pub fn match_time() -> HalResult<f64> {
    unsafe { hal_call!(HAL_GetMatchTime()).map(|time| time as f64) }
}

// what in fresh hell does this function do when the FMS is detached or the DS is offline????
// pub fn get_alliance_station() -> HalResult<AllianceStationID> {
//     unsafe { hal_call!(HAL_GetAllianceStation()) }
// }

/// Wait for new information from the driver station. `true` is returned when new data is available
/// and `false` is returned on timeout. `timeout` is the time to wait for new data in seconds.
/// Passing in `None` will wait indefinitely for new data, and always return `true`. 
pub fn wait_for_data(timeout: Option<f64>) -> bool {
    if let Some(timeout) = timeout {
        unsafe { HAL_WaitForDSDataTimeout(timeout) != 0 }
    } else {
        unsafe { HAL_WaitForDSData(); }
        true
    }
}

pub fn is_new_control_data() -> bool {
    unsafe { HAL_IsNewControlData() != 0 }
}

pub fn send_error(message: &str, location: &str, stack_trace: &str) {
    send_error_raw(true, 1, CString::new(message).unwrap(), CString::new(location).unwrap(), CString::new(stack_trace).unwrap());
}

pub fn send_warning(message: &str) {
    send_error_raw(false, 1, CString::new(message).unwrap(), CString::new("").unwrap(), CString::new("").unwrap());
}

// TODO: docs
/// Direct wrapper around `HAL_SendError`. 
pub fn send_error_raw(is_error: bool, error_code: i32, details: CString, location: CString, call_stack: CString) {
    unsafe {
        HAL_SendError(is_error as NativeBool, error_code, 0,
                      details.as_ptr(), location.as_ptr(), call_stack.as_ptr(), 1);
    }
}

pub fn initialize() { unsafe { HAL_InitializeDriverStation(); } }
pub fn observe_starting() { unsafe { HAL_ObserveUserProgramStarting(); } }
pub fn observe_disabled() { unsafe { HAL_ObserveUserProgramDisabled(); } }
pub fn observe_autonomous() { unsafe { HAL_ObserveUserProgramAutonomous(); } }
pub fn observe_teleop() { unsafe { HAL_ObserveUserProgramTeleop(); } }
pub fn observe_test() { unsafe { HAL_ObserveUserProgramTest(); } }
