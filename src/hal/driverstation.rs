use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float};
use error::*;
use types::NativeBool;

const ENABLED_BIT:      u32 = 0;
const AUTONOMOUS_BIT:   u32 = 1;
const TEST_BIT:         u32 = 1 << 1;
const STOPPED_BIT:      u32 = 1 << 2;
const FMS_ATTACHED_BIT: u32 = 1 << 3;
const DS_ATTACHED_BIT:  u32 = 1 << 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct ControlWord {
    field: u32,
}

impl ControlWord {
    #[inline(always)]
    fn get_bit(&self, mask: u32) -> bool {
        (self.field & mask) != 0
    }

    #[inline(always)] pub fn is_ds_enabled(&self) -> bool { self.get_bit(ENABLED_BIT) }
    #[inline(always)] pub fn is_autonomous(&self) -> bool { self.get_bit(AUTONOMOUS_BIT) }
    #[inline(always)] pub fn is_test(&self) -> bool { self.get_bit(TEST_BIT) }
    #[inline(always)] pub fn is_stopped(&self) -> bool { self.get_bit(STOPPED_BIT) }
    #[inline(always)] pub fn is_fms_attached(&self) -> bool { self.get_bit(FMS_ATTACHED_BIT) }
    #[inline(always)] pub fn is_ds_attached(&self) -> bool { self.get_bit(DS_ATTACHED_BIT) }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AllianceStationId {
    Red1 = 0,
    Red2 = 1,
    Red3 = 2,
    Blue1 = 3,
    Blue2 = 4,
    Blue3 = 5,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct JoystickAxes {
    pub count: i16,
    pub axes: [c_float; 12],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct JoystickPovs {
    pub count: i16,
    pub povs: [i16; 12],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct JoystickButtons {
    pub buttons: u32,
    pub count: u8,
}

#[repr(C)]
pub struct JoystickDescriptor {
    pub is_xbox: u8,
    pub type_: u8,
    pub name: [c_char; 256],
    pub axis_ount: u8,
    pub axis_types: [u8; 12],
    pub button_count: u8,
    pub pov_count: u8,
}

extern "C" {
    fn HAL_SetErrorData(errors: *const c_char, errorsLength: i32, waitMs: i32) -> i32;
    fn HAL_SendError(isError: NativeBool, errorCode: i32,
                     isLVCode: NativeBool,
                     details: *const c_char,
                     location: *const c_char,
                     callStack: *const c_char,
                     printMsg: NativeBool) -> i32;
    fn HAL_GetControlWord(controlWord: *mut ControlWord) -> i32;
    fn HAL_GetAllianceStation(status: *mut i32) -> AllianceStationId;
    fn HAL_GetJoystickAxes(joystick_num: i32, axes: *mut JoystickAxes) -> i32;
    fn HAL_GetJoystickPOVs(joystick_num: i32, povs: *mut JoystickPovs) -> i32;
    fn HAL_GetJoystickButtons(joystick_num: i32, buttons: *mut JoystickButtons) -> i32;
    fn HAL_GetJoystickDescriptor(joystick_num: i32, desc: *mut JoystickDescriptor) -> i32;
    fn HAL_GetJoystickIsXbox(joystick_num: i32) -> NativeBool;
    fn HAL_GetJoystickType(joystick_num: i32) -> i32;
    fn HAL_GetJoystickName(joystick_num: i32) -> *mut c_char;
    fn HAL_GetJoystickAxisType(joystick_num: i32, axis: i32) -> i32;
    fn HAL_SetJoystickOutputs(joystick_num: i32, outputs: i64, leftRumble: i32, rightRumble: i32) -> i32;
    fn HAL_GetMatchTime(status: *mut i32) -> c_double;
    fn HAL_WaitForDSData();
    fn HAL_InitializeDriverStation();
    fn HAL_ObserveUserProgramStarting();
    fn HAL_ObserveUserProgramDisabled();
    fn HAL_ObserveUserProgramAutonomous();
    fn HAL_ObserveUserProgramTeleop();
    fn HAL_ObserveUserProgramTest();
}

fn str_to_cstring(input: &str) -> CString {
    let mut buf = String::with_capacity(input.len());
    let mut count = 0;
    for &byte in input.as_bytes() {
        buf.push(match byte {
            b'\0' => '\u{FFFD}',
            b => b as char
        });

        count += 1;
    }

    // We just got rid of all the null bytes so we can unwrap
    CString::new(buf).unwrap()
}

#[inline(always)]
pub fn set_error_data(errors: &str, wait_ms: i32) {
    let c_string = str_to_cstring(errors);
    unsafe {
        HAL_SetErrorData(c_string.as_ptr(), errors.len() as i32, wait_ms);
    }
}

#[inline(always)]
pub fn send_error(is_error: bool, error_code: i32, is_lv_code: bool, details: &str, location: &str, call_stack: &str, print_msg: bool) {
    let details = str_to_cstring(details);
    let location = str_to_cstring(location);
    let call_stack = str_to_cstring(call_stack);
    
    unsafe {
        HAL_SendError(is_error as NativeBool, error_code, is_lv_code as NativeBool, details.as_ptr(), location.as_ptr(), call_stack.as_ptr(), print_msg as NativeBool);
    }
}

#[inline(always)]
pub fn get_control_word() -> HalResult<ControlWord> {
    unsafe {
        let mut word = ::std::mem::zeroed();
        hal_call!(ret HAL_GetControlWord(&mut word))?;
        Ok(word)
    }
}

#[inline(always)]
pub fn get_alliance_station() -> HalResult<AllianceStationId> {
    unsafe { hal_call!(ptr HAL_GetAllianceStation()) }
}

#[inline(always)]
pub fn get_joystick_axes(joystick_num: i32) -> HalResult<JoystickAxes> {
    let axes = ::std::mem::zeroed();
    unsafe { hal_call!(ret HAL_GetJoystickAxes(joystick_num, &mut axes))?; }
    Ok(axes)
}

#[inline(always)]
pub fn get_joystick_povs(joystick_num: i32) -> HalResult<JoystickPovs> {
    let povs = ::std::mem::zeroed();
    unsafe { hal_call!(ret HAL_GetJoystickPOVs(joystick_num, &mut povs))?; }
    Ok(povs)
}

#[inline(always)]
pub fn get_joystick_buttons(joystick_num: i32) -> HalResult<JoystickButtons> {
    let buttons = ::std::mem::zeroed();
    unsafe { hal_call!(ret HAL_GetJoystickButtons(joystick_num, &mut buttons))?; }
    Ok(buttons)
}

#[inline(always)]
pub fn get_joystick_descriptor(joystick_num: i32) -> HalResult<JoystickDescriptor> {
    let descriptor = ::std::mem::zeroed();
    unsafe { hal_call!(ret HAL_GetJoystickDescriptor(joystick_num, &mut descriptor))?; }
    Ok(descriptor)
}

#[inline(always)]
pub fn joystick_is_xbox(joystick_num: i32) -> bool {
    unsafe { HAL_GetJoystickIsXbox(joystick_num) != 0 }
}

#[inline(always)]
pub fn joystick_type(joystick_num: i32) -> i32 {
    unsafe { HAL_GetJoystickType(joystick_num) }
}

#[inline(always)]
pub fn joystick_name(joystick_num: i32) -> String {
    unsafe {
        use std::ffi::CStr;
        let ptr = HAL_GetJoystickName(joystick_num);
        CStr::from_ptr(ptr).to_string_lossy().to_string()
    }
}

#[inline(always)]
pub fn joystick_axis_type(joystick_num: i32, axis: i32) -> i32 {
    unsafe { HAL_GetJoystickAxisType(joystick_num, axis) }
}

#[inline(always)]
pub fn SetJoystickOutputs(joystick_num: i32, outputs: i64, left_rumble: i32, right_rumble: i32) {
    // wpilib ignores the status, so do we lol
    unsafe { HAL_SetJoystickOutputs(joystick_num, outputs, left_rumble, right_rumble); }
}

#[inline(always)]
pub fn get_match_time() {
    unsafe { hal_call!(ptr HAL_GetMatchTime()) }
}

#[inline(always)] pub fn wait_for_data() { unsafe { HAL_WaitForDSData() } }
#[inline(always)] pub fn initialize_driver_station() { unsafe { HAL_InitializeDriverStation() } }
#[inline(always)] pub fn observe_starting() { unsafe { HAL_ObserveUserProgramStarting() } }
#[inline(always)] pub fn observe_disabled() { unsafe { HAL_ObserveUserProgramDisabled() } }
#[inline(always)] pub fn observe_autonomous() { unsafe { HAL_ObserveUserProgramAutonomous() } }
#[inline(always)] pub fn observe_teleop() { unsafe { HAL_ObserveUserProgramTeleop() } }
#[inline(always)] pub fn observe_test() { unsafe { HAL_ObserveUserProgramTest() } }