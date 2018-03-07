use std::os::raw::*;
use hal::types::*;
use error::HalResult;

extern "C" {
    fn HAL_GetJoystickAxes(joystick: i32, axes: *mut Axes) -> i32;
    fn HAL_GetJoystickPOVs(joystick: i32, povs: *mut Povs) -> i32;
    fn HAL_GetJoystickButtons(joystick: i32, buttons: *mut Buttons) -> i32;
    fn HAL_GetJoystickDescriptor(joystick: i32, desc: *mut Descriptor) -> i32;
    fn HAL_GetJoystickIsXbox(joystick: i32) -> NativeBool;
    fn HAL_GetJoystickType(joystick: i32) -> i32;
    fn HAL_GetJoystickName(joystick: i32) -> *mut c_char;
    fn HAL_FreeJoystickName(name: *mut c_char);
    fn HAL_GetJoystickAxisType(joystick: i32, axis: i32) -> i32;
    fn HAL_SetJoystickOutputs(joystick: i32, outputs: i64, left_rumble: i32, right_rumble: i32) -> i32;

}

const MAX_JOYSTICK_AXES: usize = 12;
const MAX_JOYSTICK_POVS: usize = 12;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Povs {
    count: i16,
    povs: [i16; MAX_JOYSTICK_POVS]
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Buttons {
    buttons: u32,
    count: u8
}

#[repr(C)]
pub struct Descriptor {
    is_xbox: u8,
    stick_type: u8,
    name: [c_char; 256],
    _axis_count: u8,
    axis_types: [u8; MAX_JOYSTICK_AXES],
    _pov_count: u8,
    _button_count: u8,
}

impl Descriptor {
    pub fn name<'a>(&'a self) -> String {
        let slice = &self.name[..];
        let slice = unsafe { ::std::mem::transmute::<&'a [c_char], &'a [u8]>(slice) };
        String::from_utf8_lossy(slice).into_owned()
    }

    pub fn is_xbox(&self) -> bool { self.is_xbox != 0 }
    
    /// Get the "type" of the joystick, whatever that means
    pub fn get_type(&self) -> u8 { self.stick_type }

}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Axes {
    count: i16,
    axes: [c_float; MAX_JOYSTICK_AXES],
}

impl Axes {
    /// Get the value for the specified axis.
    pub fn get(&self, idx: usize) -> Option<f32> {
        if idx as i16 >= self.count { None } else {
            Some(self.axes[idx] as f32)
        }
    }
}

struct XboxController {
    numer: i32,
}

pub struct Joystick {
    pub(crate) number: i32,
}

impl Joystick {
    pub fn get_axes(&self) -> HalResult<Axes> {
        let mut axes = Axes::default();
        unsafe { hal_call!(ret HAL_GetJoystickAxes(self.number, &mut axes))? };
        Ok(axes)
    }

    pub fn get_povs(&self) -> HalResult<Povs> {
        let mut povs = Povs::default();
        unsafe { hal_call!(ret HAL_GetJoystickPOVs(self.number, &mut povs))? };
        Ok(povs)
    }

    pub fn get_buttons(&self) -> HalResult<Buttons> {
        let mut buttons = Buttons::default();
        unsafe { hal_call!(ret HAL_GetJoystickButtons(self.number, &mut buttons))? };
        Ok(buttons)
    }

    pub fn get_descriptor(&self) -> HalResult<Descriptor> {
        unsafe {
            let mut descriptor: Descriptor = ::std::mem::uninitialized();
            hal_call!(ret HAL_GetJoystickDescriptor(self.number, &mut descriptor))?;
            Ok(descriptor)
        }
    }
}
