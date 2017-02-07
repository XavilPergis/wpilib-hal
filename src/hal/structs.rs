pub trait Handle {
    fn new(port: i32) -> Self;
    fn get_handle(&self) -> i32;
    fn transmute_handle<H>(&self) -> H where H: Self {
        H::new(self.get_handle())
    }
}

pub struct AnalogInputHandle(i32);
pub struct GyroHandle(i32);
pub struct PortHandle(i32);
pub struct AnalogOutputHandle(i32);
pub struct AnalogTriggerHandle(i32);
pub struct CompressorHandle(i32);
pub struct CounterHandle(i32);
pub struct DigitalHandle(i32);
pub struct DigitalPwmHandle(i32);
pub struct RelayHandle(i32);
pub struct SolenoidHandle(i32);
pub struct InterruptHandle(i32);
pub struct NotifierHandle(i32);

impl Handle for AnalogInputHandle   {
    fn new(port: i32) -> AnalogInputHandle { AnalogInputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for GyroHandle          {
    fn new(port: i32) -> GyroHandle { GyroHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for PortHandle          {
    fn new(port: i32) -> PortHandle { PortHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogOutputHandle  {
    fn new(port: i32) -> AnalogOutputHandle { AnalogOutputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogTriggerHandle {
    fn new(port: i32) -> AnalogTriggerHandle { AnalogTriggerHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogOutputHandle  {
    fn new(port: i32) -> AnalogOutputHandle { AnalogOutputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for CompressorHandle    {
    fn new(port: i32) -> CompressorHandle { CompressorHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for CounterHandle       {
    fn new(port: i32) -> CounterHandle { CounterHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for DigitalHandle       {
    fn new(port: i32) -> DigitalHandle { DigitalHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for DigitalPwmHandle    {
    fn new(port: i32) -> DigitalPwmHandle { DigitalPwmHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for RelayHandle         {
    fn new(port: i32) -> RelayHandle { RelayHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for SolenoidHandle      {
    fn new(port: i32) -> SolenoidHandle { SolenoidHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for InterruptHandle     {
    fn new(port: i32) -> InterruptHandle { InterruptHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for NotifierHandle      {
    fn new(port: i32) -> NotifierHandle { NotifierHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}


// macro_rules! gen_handles {
//    ($($typ:ident),*) => (
//        $(
//            pub struct $typ(i32);
//            impl Handle for $typ {
//                fn get_handle(&self) -> i32 { self.0 }
//            }
//        )*
//    );
// }

// gen_handles! {
//    AnalogInputHandle
//    GyroHandle,
//    PortHandle,
//    AnalogOutputHandle,
//    AnalogTriggerHandle,
//    AnalogOutputHandle,
//    CompressorHandle,
//    CounterHandle,
//    DigitalHandle,
//    DigitalPwmHandle,
//    RelayHandle,
//    SolenoidHandle,
//    InterruptHandle
// }

// TODO: More insightful comments
/// What modes the driver station is in. Only one of `enabled`, `autonomous`, `test`, and `stopped`
/// should be on at a time.
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
    pub ds_attached: bool
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

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct JoystickPovs {
    /// Turns out, each element is actually the angle of the POV in degrees.
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
            stopped: false,
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
            stopped: raw.eStop() != 0,
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
