use error::*;
use hal::types::*;
use std::os::raw::*;

extern "C" {
    fn HAL_InitializeDIOPort(handle: PortHandle, input: NativeBool, status: *mut i32) -> DigitalHandle;
    fn HAL_CheckDIOChannel(channel: i32) -> NativeBool;
    fn HAL_FreeDIOPort(handle: DigitalHandle);
    fn HAL_SetDIO(handle: DigitalHandle, value: NativeBool, status: *mut i32);
    fn HAL_GetDIO(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    // fn HAL_SetDIODirection(handle: DigitalHandle, input: NativeBool, status: *mut i32);
    // fn HAL_GetDIODirection(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    fn HAL_Pulse(handle: DigitalHandle, pulse_length: c_double, status: *mut i32);
    fn HAL_IsPulsing(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    // fn HAL_IsAnyPulsing(status: *mut i32) -> NativeBool;
    fn HAL_SetFilterSelect(handle: DigitalHandle, filter_index: i32, status: *mut i32);
    fn HAL_GetFilterSelect(handle: DigitalHandle, status: *mut i32) -> i32;
    fn HAL_SetFilterPeriod(filter_index: i32, value: i64, status: *mut i32);
    fn HAL_GetFilterPeriod(filter_index: i32, status: *mut i32) -> i64;

    fn HAL_AllocateDigitalPWM(status: *mut i32) -> DigitalPwmHandle;
    fn HAL_FreeDigitalPWM(pwm: DigitalPwmHandle, status: *mut i32);
    fn HAL_SetDigitalPWMRate(rate: c_double, status: *mut i32);
    fn HAL_SetDigitalPWMDutyCycle(pwm: DigitalPwmHandle, duty_cycle: c_double, status: *mut i32);
    fn HAL_SetDigitalPWMOutputChannel(pwm: DigitalPwmHandle, channel: i32, status: *mut i32);
}

fn check_digital_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

#[derive(Debug)]
struct DigitalIO {
    handle: Handle,
    channel: i32,
}

impl DigitalIO {
    fn new(channel: i32, input: bool) -> HalResult<Self> {
        if !check_digital_channel(channel) { return Err(HalError::InvalidChannel(channel)); }
        let port = ::hal::get_port(channel).ok_or(HalError::InvalidChannel(channel))?;

        unsafe {
            hal_call!(HAL_InitializeDIOPort(port, input as NativeBool))
                .map(|handle| DigitalIO { handle, channel })
        }
    }
}

impl Drop for DigitalIO {
    fn drop(&mut self) {
        unsafe { HAL_FreeDIOPort(self.handle); }
    }
}

#[derive(Debug)]
pub struct DigitalInput {
    dio: DigitalIO
}

impl DigitalInput {
    pub fn new(channel: i32) -> HalResult<Self> {
        Ok(DigitalInput { dio: DigitalIO::new(channel, true)? })
    }

    pub fn get(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetDIO(self.dio.handle)).map(|val| val != 0) }
    }
}

#[derive(Debug)]
pub struct DigitalOutput {
    dio: DigitalIO
}

impl DigitalOutput {
    pub fn new(channel: i32) -> HalResult<Self> {
        Ok(DigitalOutput { dio: DigitalIO::new(channel, false)? })
    }

    pub fn as_pwm_output(&mut self) -> HalResult<PwmGenerator> { PwmGenerator::new(self) }

    pub fn get(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetDIO(self.dio.handle)).map(|val| val != 0) }
    }

    pub fn set(&self, value: bool) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetDIO(self.dio.handle, value as NativeBool)) }
    }

    pub fn pulse(&self, length: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_Pulse(self.dio.handle, length as c_double)) }
    }

    pub fn is_pulsing(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_IsPulsing(self.dio.handle)).map(|val| val != 0) }
    }
}

/// Struct to generate a PWM signal on a digital output rather than controlling on/off manually.
#[derive(Debug)]
pub struct PwmGenerator<'out> {
    _output: &'out DigitalOutput,
    pwm_handle: Handle
}

impl<'out> PwmGenerator<'out> {
    fn new(output: &'out DigitalOutput) -> HalResult<Self> {
        unsafe {
            // a digital pwm generator is a digital output
            let pwm_handle = hal_call!(HAL_AllocateDigitalPWM())?;
            // Point the PWM generator at our digital output
            hal_call!(HAL_SetDigitalPWMOutputChannel(pwm_handle, output.dio.channel))?;

            Ok(PwmGenerator { _output: output, pwm_handle })
        }
    }

    /// Set the global PWM generator rate.
    pub fn set_rate(rate: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetDigitalPWMRate(rate as c_double)) }
    }

    /// Set the duty cycle. Panics if `duty_cycle` is not in 0..1.
    pub fn set_duty_cycle(&self, duty_cycle: f64) -> HalResult<()> {
        if duty_cycle > 1.0 || duty_cycle < 0.0 {
            panic!("Duty cycle out of range. Expected range is [0, 1], but the actual value was {}", duty_cycle);
        }

        unsafe { hal_call!(HAL_SetDigitalPWMDutyCycle(self.pwm_handle, duty_cycle as c_double)) }
    }
}

impl<'out> Drop for PwmGenerator<'out> {
    fn drop(&mut self) {
        // Unused status param
        unsafe { HAL_FreeDigitalPWM(self.pwm_handle, ::std::ptr::null_mut()) }
    }
}