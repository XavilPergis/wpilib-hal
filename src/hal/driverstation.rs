//! This module ports WPILIB's `HAL/DriverStation.h` to Rust

use raw::*;
use hal::error::{ HalResult, HalError };

use std::ffi::CString;
use std::mem;

use time::Duration;

pub const MAX_JOYSTICK_AXES: usize = 12;
pub const MAX_JOYSTICK_POVS: usize = 12;

/// Where the driver station is on the field. Either `Red` or `Blue` with a position from 1 to 3
/// or an invalid station.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllianceStation {
    Red(u8), Blue(u8), Invalid
}

// TODO: More insightful comments
/// What modes the driver station is in. Only one of `enabled`, `autonomous`, `test`, and `e_stop`
/// should be on at a time.
///
/// ## Fields
/// * `enabled` - Whether the driver station is enabled
/// * `autonomous` - Whether the driver station is in autonomous mode
/// * `test` - Whether the driver station is in test mode
/// * `e_stop` - Whether the driver station is stopped
/// * `fms_attached` - Whether the Field Managment System is attached
/// * `ds_attached` - Whether the Driver STation is attached
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ControlWord {
    pub enabled: bool,
    pub autonomous: bool,
    pub test: bool,
    pub e_stop: bool,
    pub fms_attached: bool,
    pub ds_attached: bool
}

/// TODO: Figure out what a joystick type actually is
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JoystickType {
    Unknown(i32)
}

// TODO: Model of the joystick?
/// Represents the rotation on different "axes" of a joystick. On *our* joysticks:
///
/// * 1: Rotation in the x direction
/// * 2: Rotation in the y direction
/// * 3: Paddle at bottom of joystick
/// * 4: Twist of the stick
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JoystickAxes {
    axes: [f32; MAX_JOYSTICK_AXES],
    count: i16
}

impl From<HAL_JoystickAxes> for JoystickAxes {
    fn from(raw_axes: HAL_JoystickAxes) -> JoystickAxes {
        JoystickAxes {
            axes: raw_axes.axes,
            count: raw_axes.count
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct JoystickPovs {
    povs: [i16; MAX_JOYSTICK_POVS],
    count: i16
}

impl From<HAL_JoystickPOVs> for JoystickPovs {
    fn from(raw_povs: HAL_JoystickPOVs) -> JoystickPovs {
        JoystickPovs {
            povs: raw_povs.povs,
            count: raw_povs.count
        }
    }
}

/// Represents what buttons are pressed on the joystick
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoystickButtons {
    // This field is actually an i32 bitmask in the wpilib headers
    // An i32 is 32 bit; it can hold 32 different buttons
    buttons_down: [bool; 32],
    // TODO: Leaving this in until I know what it does
    count: u16
}

/// What mode the user program is running in.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserProgramMode {
    Starting,
    Disabled,
    Autonomous,
    Teleop,
    Test
}

/// A struct describing a joystick.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JoystickDescriptor {
    name: String,
    is_xbox: bool,
    stick_type: JoystickType,
    button_count: u8,
    axis_count: u8,
    axis_types: [u8; MAX_JOYSTICK_AXES],
    pov_count: u8
}

impl ControlWord {
    /// Returns a new `ControlWord` with all the fields set to false.
    #[inline]
    pub fn all_off() -> ControlWord {
        ControlWord {
            enabled: false,
            autonomous: false,
            test: false,
            e_stop: false,
            fms_attached: false,
            ds_attached: false
        }
    }
}

impl From<HAL_ControlWord> for ControlWord {
    #[inline]
    fn from(raw: HAL_ControlWord) -> ControlWord {
        ControlWord {
            enabled: raw.enabled() != 0,
            autonomous: raw.autonomous() != 0,
            test: raw.test() != 0,
            e_stop: raw.eStop() != 0,
            fms_attached: raw.fmsAttached() != 0,
            ds_attached: raw.dsAttached() != 0
        }
    }
}

impl From<i32> for JoystickType {
    fn from(joystick_type: i32) -> JoystickType {
        // TODO: Figure out what the hell the joystick type means
        match joystick_type {
            k => JoystickType::Unknown(k)
        }
    }
}

impl JoystickButtons {
    #[inline]
    pub fn count(&self) -> u16 {
        // HAL should never give us more buttons than can fit in a u16
        self.buttons_down.len() as u16
    }
}

impl From<HAL_JoystickButtons> for JoystickButtons {
    fn from(raw_buttons: HAL_JoystickButtons) -> JoystickButtons {
        let mut buttons = JoystickButtons {
            buttons_down: [false; 32],
            count: 0
        };

        for i in 0..32 {
            // extract a field from the i32 bitmask
            buttons.buttons_down[i] = raw_buttons.buttons & (1 << i) == 1;
            buttons.count += 1;
        }

        buttons
    }
}

impl From<HAL_JoystickDescriptor> for JoystickDescriptor {
    fn from(raw_descriptor: HAL_JoystickDescriptor) -> JoystickDescriptor {
        JoystickDescriptor {
            // FIXME: Does this even work?
            name: String::from_utf8_lossy(raw_descriptor.name.iter().map(|x| *x as u8).collect::<Vec<u8>>().as_slice()).escape_default(),
            is_xbox: raw_descriptor.isXbox != 0,
            stick_type: JoystickType::from(raw_descriptor.type_ as i32),
            button_count: raw_descriptor.buttonCount,
            axis_types: raw_descriptor.axisTypes,
            axis_count: raw_descriptor.axisCount,
            pov_count: raw_descriptor.povCount
        }
    }
}

pub fn set_error_data(errors: &str, errors_length: i32, wait_ms: i32) -> HalResult<()> {
    hal_status_return_call!(HAL_SetErrorData(CString::new(errors).map_err(|err| HalError::from(err))?.as_ptr(), errors_length, wait_ms))
}

pub fn get_joystick_descriptor(joystick_num: i32) -> HalResult<JoystickDescriptor> {
    let mut descriptor = unsafe { mem::uninitialized() };
    hal_status_return_call!(HAL_GetJoystickDescriptor(joystick_num, &mut descriptor as *mut HAL_JoystickDescriptor))?;

    Ok(JoystickDescriptor::from(descriptor))
}

pub fn get_joystick_axes(joystick_num: i32) -> HalResult<JoystickAxes> {
    let mut raw_axes = unsafe { mem::uninitialized() };
    hal_status_return_call!(HAL_GetJoystickAxes(joystick_num, &mut raw_axes as *mut HAL_JoystickAxes))?;

    Ok(JoystickAxes::from(raw_axes))
}

pub fn get_joystick_povs(joystick_num: i32) -> HalResult<JoystickPovs> {
    let mut raw_povs = unsafe { mem::uninitialized() };
    hal_status_return_call!(HAL_GetJoystickPOVs(joystick_num, &mut raw_povs as *mut HAL_JoystickPOVs))?;

    Ok(JoystickPovs::from(raw_povs))
}

pub fn get_joystick_buttons(joystick_num: i32) -> HalResult<JoystickButtons> {
    let mut raw_buttons: HAL_JoystickButtons = unsafe { mem::uninitialized() };
    hal_status_return_call!(HAL_GetJoystickButtons(joystick_num, &mut raw_buttons as *mut HAL_JoystickButtons))?;

    Ok(JoystickButtons::from(raw_buttons))
}

pub fn get_joystick_is_xbox(joystick_num: i32) -> HalResult<bool> {
    Ok(get_joystick_descriptor(joystick_num)?.is_xbox)
}

pub fn get_joystick_type(joystick_num: i32) -> HalResult<JoystickType> {
    Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.stick_type))
}

pub fn get_joystick_name(joystick_num: i32) -> HalResult<String> {
    Ok(get_joystick_descriptor(joystick_num)?.name)
}

pub fn get_joystick_axis_type(joystick_num: i32, axis: i32) -> HalResult<JoystickType> {
    if axis >= 0 {
        Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.axis_types[axis as usize] as i32))
    } else {
        Err(HalError::IndexOutOfRange)
    }
}

/// TODO: What is this?
pub fn set_joystick_outputs(joystick_num: i32, outputs: i64, left_rumble: i32, right_rumble: i32) -> HalResult<()> {
    hal_status_return_call!(HAL_SetJoystickOutputs(joystick_num, outputs, left_rumble, right_rumble))
}

/// TODO: What are we actually observing? This should be called in the main DS loop
pub fn observe_user_program(mode: UserProgramMode) {
    unsafe {
        match mode {
            UserProgramMode::Starting   => HAL_ObserveUserProgramStarting(),
            UserProgramMode::Disabled   => HAL_ObserveUserProgramDisabled(),
            UserProgramMode::Autonomous => HAL_ObserveUserProgramAutonomous(),
            UserProgramMode::Teleop     => HAL_ObserveUserProgramTeleop(),
            UserProgramMode::Test       => HAL_ObserveUserProgramTest()
        }
    }
}

/// Initialize the driver station. Should only be called once.
pub fn initialize_driver_station() {
    unsafe { HAL_InitializeDriverStation() };
}

/// Gets a control word directly from the driver station. The result should be cached for ~50ms
pub fn get_control_word() -> HalResult<ControlWord> {
    let mut control_word: HAL_ControlWord = unsafe { mem::uninitialized() };
    hal_status_return_call!(HAL_GetControlWord(&mut control_word as *mut HAL_ControlWord))?;

    Ok(ControlWord::from(control_word))
}

/// Blocks until the DS returns some data. Good for building concurrent abstractions.
///
/// ## Example
/// ```
/// let (tx, rx) = mpsc::channel::<i32>();
/// thread::spawn(|| {
///     let thread_tx = tx.clone();
///     let count = 0;
///     loop {
///         wait_for_ds_data();
///         thread_tx.send(count).unwrap();
///         count += 1;
///     }
/// });
/// ```
pub fn wait_for_ds_data() {
    unsafe { HAL_WaitForDSData() };
}

/// Reports an error to the driver station
pub fn send_error(is_error: bool, error_code: i32, is_lv_code: bool, details: &str, location: &str, call_stack: &str, print_message: bool) -> Result<(), HalError> {
    // CString::new() will return an `Err(NulError)` if there is a `\0` in the string passed in
    // Since this is a struct type error, it means that only `Err(NulError)` should ever be passed
    // in so we can safely transmute `NulError` into `HalError::NullError`
    let details_raw = CString::new(details).map_err(|err| HalError::from(err))?;
    let location_raw = CString::new(location).map_err(|err| HalError::from(err))?;
    let call_stack_raw = CString::new(call_stack).map_err(|err| HalError::from(err))?;

    // TODO: Will the pointers be dropped here? I don't *think* so?
    hal_status_return_call!(HAL_SendError(is_error as HAL_Bool, error_code, is_lv_code as HAL_Bool,
        details_raw.as_ptr(), location_raw.as_ptr(), call_stack_raw.as_ptr(),
        print_message as HAL_Bool))
}

/// returns where the driver station thinks it is.
pub fn get_alliance_station() -> HalResult<AllianceStation> {
    let station_id = hal_status_pointer_call!(HAL_GetAllianceStation())?;

    use self::HAL_AllianceStationID::*;

    Ok(match station_id {
        HAL_AllianceStationID_kRed1  => AllianceStation::Red(1),
        HAL_AllianceStationID_kRed2  => AllianceStation::Red(2),
        HAL_AllianceStationID_kRed3  => AllianceStation::Red(3),
        HAL_AllianceStationID_kBlue1 => AllianceStation::Blue(1),
        HAL_AllianceStationID_kBlue2 => AllianceStation::Blue(2),
        HAL_AllianceStationID_kBlue3 => AllianceStation::Blue(3)
    })
}

/// Get the match time so far. This is not the *actual* match time, just an approximation of it.
/// Since this is not the canonical match time, it cannot be used to dispute times or garuntee
/// that a task completes before the match runs out.
pub fn get_match_time_approx() -> HalResult<Duration> {
    let time = hal_status_pointer_call!(HAL_GetMatchTime())?;

    // TODO: What the hell are the units that are returned!? Probably seconds...
    let time_ns = (time * 1_000_000_000f64) as i64;
    Ok(Duration::nanoseconds(time_ns))
}
