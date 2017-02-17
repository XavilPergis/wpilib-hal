//! This module ports WPILIB's `HAL/DriverStation.h` to Rust

use ::error::*;
use raw::*;

use std::ffi::CString;
use std::mem;
use std::ops::Index;

use time::Duration;

#[allow(missing_docs)] pub type RawControlWord = HAL_ControlWord;
#[allow(missing_docs)] pub type RawJoystickAxes = HAL_JoystickAxes;
#[allow(missing_docs)] pub type RawJoystickPovs = HAL_JoystickPOVs;
#[allow(missing_docs)] pub type RawJoystickButtons = HAL_JoystickButtons;
#[allow(missing_docs)] pub type RawJoystickDescriptor = HAL_JoystickDescriptor;

/// The maximum amount of axes that a controller can have. Realistically, this
/// is 3 or 4.
pub const MAX_JOYSTICK_AXES: usize = 12;
/// The maximum amount of POVs that a controller can have. POVs are the little
/// D-pad like things on the top of the joystick.
pub const MAX_JOYSTICK_POVS: usize = 12;

// TODO: More insightful comments
/// What modes the driver station is in. Only one of `enabled`, `autonomous`,
/// `test`, and `stopped` should be on at a time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ControlWord {
    /// Whether the driver station is enabled
    pub enabled: bool,
    /// Whether the driver station is in autonomous mode
    pub autonomous: bool,
    /// Whether the driver station is in test mode
    pub test: bool,
    /// Whether the driver station is stopped
    pub stopped: bool,
    /// Whether the Field Managment System is attached
    pub fms_attached: bool,
    /// Whether the Driver STation is attached
    pub ds_attached: bool,
}

impl ControlWord {
    /// Returns a new `ControlWord` with all the fields set to false.
    #[inline]
    pub fn all_off() -> ControlWord {
        ControlWord {
            enabled: false,
            autonomous: false,
            test: false,
            stopped: false,
            fms_attached: false,
            ds_attached: false,
        }
    }
}

const ENABLED_OFFSET:      i32 = 0;
const AUTONOMOUS_OFFSET:   i32 = 1;
const TEST_OFFSET:         i32 = 2;
const STOPPED_OFFSET:      i32 = 3;
const FMS_ATTACHED_OFFSET: i32 = 4;
const DS_ATTACHED_OFFSET:  i32 = 5;

fn extract_field(word: RawControlWord, offset: i32) -> u32 {
    (word._bitfield_1 & (2 << offset - 1)) >> offset
}

impl From<RawControlWord> for ControlWord {
    #[inline]
    fn from(raw: RawControlWord) -> ControlWord {
        ControlWord {
            enabled:      extract_field(raw, ENABLED_OFFSET) != 0,
            autonomous:   extract_field(raw, AUTONOMOUS_OFFSET) != 0,
            test:         extract_field(raw, TEST_OFFSET) != 0,
            stopped:      extract_field(raw, STOPPED_OFFSET) != 0,
            fms_attached: extract_field(raw, FMS_ATTACHED_OFFSET) != 0,
            ds_attached:  extract_field(raw, DS_ATTACHED_OFFSET) != 0,
        }
    }
}

/// Represents the rotation on different "axes" of a joystick.
///
/// On Extreme 3D Pro joysticks:
/// * 1: Rotation in the x direction
/// * 2: Rotation in the y direction
/// * 3: Paddle at bottom of joystick
/// * 4: Twist of the stick
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JoystickAxes {
    axes: [f32; MAX_JOYSTICK_AXES],
    count: i16,
}

impl Index<usize> for JoystickAxes {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axes[index]
    }
}

impl From<RawJoystickAxes> for JoystickAxes {
    fn from(raw_axes: RawJoystickAxes) -> JoystickAxes {
        JoystickAxes {
            axes: raw_axes.axes,
            count: raw_axes.count,
        }
    }
}

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct JoystickPovs {
    /// Turns out, each element is actually the angle of the POV in degrees.
    povs: [i16; MAX_JOYSTICK_POVS],
    count: i16,
}

impl Index<usize> for JoystickPovs {
    type Output = i16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.povs[index]
    }
}

impl From<RawJoystickPovs> for JoystickPovs {
    fn from(raw_povs: RawJoystickPovs) -> JoystickPovs {
        JoystickPovs {
            povs: raw_povs.povs,
            count: raw_povs.count,
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
    pub count: u16,
}

impl Index<usize> for JoystickButtons {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buttons_down[index]
    }
}

impl From<RawJoystickButtons> for JoystickButtons {
    fn from(raw_buttons: RawJoystickButtons) -> JoystickButtons {
        let mut buttons = JoystickButtons {
            buttons_down: [false; 32],
            count: 0,
        };

        for i in 0 .. 32 {
            // extract a field from the i32 bitmask
            buttons.buttons_down[i] = raw_buttons.buttons & (1 << i) == 1;
            buttons.count += 1;
        }

        buttons
    }
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
    pov_count: u8,
}

impl From<RawJoystickDescriptor> for JoystickDescriptor {
    fn from(raw_descriptor: RawJoystickDescriptor) -> JoystickDescriptor {
        JoystickDescriptor {
            // FIXME: Does this even work?
            name: String::from_utf8_lossy(raw_descriptor.name
                    .iter()
                    .map(|x| *x as u8)
                    .collect::<Vec<u8>>()
                    .as_slice())
                .escape_default(),
            is_xbox: raw_descriptor.isXbox != 0,
            stick_type: JoystickType::from(raw_descriptor.type_ as i32),
            button_count: raw_descriptor.buttonCount,
            axis_types: raw_descriptor.axisTypes,
            axis_count: raw_descriptor.axisCount,
            pov_count: raw_descriptor.povCount,
        }
    }
}

/// Where the driver station is on the field. Either `Red` or `Blue` with a
/// position from 1 to 3
/// or an invalid station.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllianceStation {
    /// Red alliance station
    Red(u8),
    /// Blue alliance station
    Blue(u8),
    /// Invalid station
    Invalid,
}

/// TODO: Figure out what a joystick type actually is
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JoystickType {
    /// An unknown type of joystick. TODO: What is this?
    Unknown(i32),
}

impl From<i32> for JoystickType {
    fn from(joystick_type: i32) -> JoystickType {
        // TODO: Figure out what the hell the joystick type means
        match joystick_type {
            k => JoystickType::Unknown(k),
        }
    }
}

/// What mode the user program is running in.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserProgramMode {
    /// The user program is starting
    Starting,
    /// The user program is disabled
    Disabled,
    /// The user program is in autonomous mode
    Autonomous,
    /// The user proram is in tele-operated mode
    TeleOperated,
    /// The user program is in test mode
    Test,
}

// TODO: What is this?
pub fn set_error_data(errors: &str, errors_length: i32, wait_ms: i32) -> HalResult<()> {
    unsafe { hal_call![ ret HAL_SetErrorData(CString::new(errors).map_err(HalError::from)?.as_ptr(), errors_length, wait_ms) ] }
}

/// Gets a joystick's descriptor from the driver station
pub fn get_joystick_descriptor(joystick_num: i32) -> HalResult<JoystickDescriptor> {
    let mut descriptor = unsafe { mem::zeroed() };
    unsafe { hal_call!(ret HAL_GetJoystickDescriptor(joystick_num, &mut descriptor as *mut RawJoystickDescriptor))?; }

    Ok(JoystickDescriptor::from(descriptor))
}

/// Gets the rotations on each "axis" of a joystick. An axis is basically just
/// something that can
/// be somewhere in a range of values.
pub fn get_joystick_axes(joystick_num: i32) -> HalResult<JoystickAxes> {
    let mut raw_axes = unsafe { mem::zeroed() };
    unsafe { hal_call!(ret HAL_GetJoystickAxes(joystick_num, &mut raw_axes as *mut RawJoystickAxes))?; }

    Ok(JoystickAxes::from(raw_axes))
}

/// Gets the state of all the POVs on the joystick.
pub fn get_joystick_povs(joystick_num: i32) -> HalResult<JoystickPovs> {
    let mut raw_povs = unsafe { mem::zeroed() };
    unsafe { hal_call!(ret HAL_GetJoystickPOVs(joystick_num, &mut raw_povs as *mut RawJoystickPovs))?; }

    Ok(JoystickPovs::from(raw_povs))
}

/// Gets what buttons are pressed on a joystick
pub fn get_joystick_buttons(joystick_num: i32) -> HalResult<JoystickButtons> {
    let mut raw_buttons: RawJoystickButtons = unsafe { mem::zeroed() };
    unsafe { hal_call!(ret HAL_GetJoystickButtons(joystick_num, &mut raw_buttons as *mut RawJoystickButtons))?; }

    Ok(JoystickButtons::from(raw_buttons))
}

/// Gets whether the joystick is an xbox controller or not
pub fn get_joystick_is_xbox(joystick_num: i32) -> HalResult<bool> {
    Ok(get_joystick_descriptor(joystick_num)?.is_xbox)
}

/// TODO: Figure out what a joystick type is
pub fn get_joystick_type(joystick_num: i32) -> HalResult<JoystickType> {
    Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.stick_type))
}

/// Gets the name of a joystick. This will return a string with a length no
/// greater than 256.
pub fn get_joystick_name(joystick_num: i32) -> HalResult<String> {
    Ok(get_joystick_descriptor(joystick_num)?.name)
}

// TODO: Figure out what a joystick type is
pub fn get_joystick_axis_type(joystick_num: i32, axis: i32) -> HalResult<JoystickType> {
    if axis >= 0 {
        Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.axis_types[axis as usize] as
                              i32))
    } else {
        Err(HalError::Hal(FfiError::ParameterOutOfRange))
    }
}

// TODO: What is this?
pub fn set_joystick_outputs(joystick_num: i32, outputs: i64, left_rumble: i32, right_rumble: i32)
                            -> HalResult<()> {
    unsafe { hal_call!(ret HAL_SetJoystickOutputs(joystick_num, outputs, left_rumble, right_rumble)) }
}

// TODO: What are we actually observing? This should be called in the main DS
// loop
pub fn observe_user_program(mode: UserProgramMode) {
    match mode {
        UserProgramMode::Starting => self::observe_user_program_starting(),
        UserProgramMode::Disabled => self::observe_user_program_disabled(),
        UserProgramMode::Autonomous => self::observe_user_program_autonomous(),
        UserProgramMode::TeleOperated => self::observe_user_program_teleop(),
        UserProgramMode::Test => self::observe_user_program_test(),
    }
}

/// Initialize the driver station. Should only be called once.
pub fn initialize_driver_station() {
    unsafe { HAL_InitializeDriverStation() };
}

/// Gets a control word directly from the driver station. The result should be
/// cached for ~50ms
pub fn get_control_word() -> HalResult<ControlWord> {
    let mut control_word: RawControlWord = unsafe { mem::zeroed() };
    unsafe { hal_call!(ret HAL_GetControlWord(&mut control_word as *mut RawControlWord))?; }

    Ok(ControlWord::from(control_word))
}

/// Blocks until the DS returns some data. Good for building concurrent
/// abstractions.
///
/// ## Example
/// ```
/// // create a channel for doing concurrent locking
/// let (tx, rx) = mpsc::channel::<()>();
/// thread::spawn(|| {
///     let thread_tx = tx.clone();
///     loop {
///         // Wait for the DS data to update
///         wait_for_ds_data();
///         // And then send an "unlock" to the receiver
///         thread_tx.send(()).unwrap();
///     }
/// });
/// ```
pub fn wait_for_ds_data() {
    unsafe { HAL_WaitForDSData() };
}

/// Reports an error to the driver station
pub fn send_error(is_error: bool, error_code: i32, is_lv_code: bool, details: &str,
                  location: &str, call_stack: &str, print_message: bool)
                  -> Result<(), HalError> {
    // CString::new() will return an `Err(NulError)` if there is a `\0` in the
    // string passed in
    // Since this is a struct type error, it means that only `Err(NulError)` should
    // ever be passed
    // in so we can safely transmute `NulError` into `HalError::NullError`
    let details_raw = CString::new(details).map_err(HalError::from)?;
    let location_raw = CString::new(location).map_err(HalError::from)?;
    let call_stack_raw = CString::new(call_stack).map_err(HalError::from)?;

    // TODO: Will the pointers be dropped here? I don't *think* so?
    unsafe { hal_call!(ret HAL_SendError(is_error as HAL_Bool, error_code, is_lv_code as HAL_Bool,
    details_raw.as_ptr(), location_raw.as_ptr(), call_stack_raw.as_ptr(),
    print_message as HAL_Bool)) }
}

/// Gets where the driver station thinks it is.
pub fn get_alliance_station() -> HalResult<AllianceStation> {
    let station_id = unsafe { hal_call!(ptr HAL_GetAllianceStation())? };

    use raw::HAL_AllianceStationID;

    Ok(match station_id {
        HAL_AllianceStationID::HAL_AllianceStationID_kRed1 => AllianceStation::Red(1),
        HAL_AllianceStationID::HAL_AllianceStationID_kRed2 => AllianceStation::Red(2),
        HAL_AllianceStationID::HAL_AllianceStationID_kRed3 => AllianceStation::Red(3),
        HAL_AllianceStationID::HAL_AllianceStationID_kBlue1 => AllianceStation::Blue(1),
        HAL_AllianceStationID::HAL_AllianceStationID_kBlue2 => AllianceStation::Blue(2),
        HAL_AllianceStationID::HAL_AllianceStationID_kBlue3 => AllianceStation::Blue(3),
    })
}

/// Gets the match time so far. This is not the *actual* match time, just an
/// approximation of it.
/// Since this is not the canonical match time, it cannot be used to dispute
/// times or garuntee
/// that a task completes before the match runs out.
pub fn get_match_time_approx() -> HalResult<Duration> {
    let time = unsafe { hal_call![ ptr HAL_GetMatchTime() ]? };

    // TODO: What the hell are the units that are returned!? Probably seconds...
    let time_ns = (time * 1_000_000_000f64) as i64;
    Ok(Duration::nanoseconds(time_ns))
}

pub fn observe_user_program_starting() {
    unsafe { HAL_ObserveUserProgramStarting() }
}

pub fn observe_user_program_disabled() {
    unsafe { HAL_ObserveUserProgramDisabled() }
}

pub fn observe_user_program_autonomous() {
    unsafe { HAL_ObserveUserProgramAutonomous() }
}

pub fn observe_user_program_teleop() {
    unsafe { HAL_ObserveUserProgramTeleop() }
}

pub fn observe_user_program_test() {
    unsafe { HAL_ObserveUserProgramTest() }
}
