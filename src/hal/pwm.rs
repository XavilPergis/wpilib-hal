use error::*;
use hal::types::*;
use std::os::raw::*;

extern "C" {
    fn HAL_InitializePWMPort(handle: PortHandle, status: *mut i32) -> DigitalHandle;
    fn HAL_FreePWMPort(handle: DigitalHandle, status: *mut i32);
    fn HAL_CheckPWMChannel(channel: i32) -> NativeBool;
    fn HAL_SetPWMConfig(handle: DigitalHandle,
                        maxPwm: c_double,
                        deadbandMaxPwm: c_double,
                        centerPwm: c_double,
                        deadbandMinPwm: c_double,
                        minPwm: c_double,
                        status: *mut i32);
    fn HAL_SetPWMConfigRaw(handle: DigitalHandle, maxPwm: i32,
                           deadbandMaxPwm: i32, centerPwm: i32,
                           deadbandMinPwm: i32, minPwm: i32,
                           status: *mut i32);
    fn HAL_GetPWMConfigRaw(handle: DigitalHandle,
                           maxPwm: *mut i32, deadbandMaxPwm: *mut i32,
                           centerPwm: *mut i32, deadbandMinPwm: *mut i32,
                           minPwm: *mut i32, status: *mut i32);
    fn HAL_SetPWMEliminateDeadband(handle: DigitalHandle, eliminateDeadband: NativeBool, status: *mut i32);
    fn HAL_GetPWMEliminateDeadband(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetPWMRaw(handle: DigitalHandle, value: i32, status: *mut i32);
    fn HAL_SetPWMSpeed(handle: DigitalHandle, speed: c_double, status: *mut i32);
    fn HAL_SetPWMPosition(handle: DigitalHandle, position: c_double, status: *mut i32);
    fn HAL_SetPWMDisabled(handle: DigitalHandle, status: *mut i32);
    fn HAL_GetPWMRaw(handle: DigitalHandle, status: *mut i32) -> i32;
    fn HAL_GetPWMSpeed(handle: DigitalHandle, status: *mut i32) -> c_double;
    fn HAL_GetPWMPosition(handle: DigitalHandle, status: *mut i32) -> c_double;
    fn HAL_LatchPWMZero(handle: DigitalHandle, status: *mut i32);
    fn HAL_SetPWMPeriodScale(handle: DigitalHandle, squelchMask: i32, status: *mut i32);
    fn HAL_GetLoopTiming(status: *mut i32) -> i32;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PwmConfig {
    pub max_pwm: i32,
    pub deadband_max_pwm: i32,
    pub center_pwm: i32,
    pub deadband_min_pwm: i32,
    pub min_pwm: i32,
}

#[derive(Debug)]
pub struct Pwm {
    pub(crate) handle: Handle,
}

impl Pwm {
    fn initialize(port: PortHandle) -> HalResult<Self> {
        unsafe { hal_call!(HAL_InitializePWMPort(port)).map(|handle| Pwm { handle }) }
    }
}

impl Drop for Pwm {
    fn drop(&mut self) {
        // Ok this one can fail, but what are we supposed to do other than panic here?
        // unsafe { HAL_FreePWMPort(self.handle) }
    }
}
