use error::*;
use hal::types::{PortHandle, DigitalHandle, DigitalPwmHandle, NativeBool};

extern "C" {
    fn HAL_InitializeDIOPort(handle: PortHandle, input: NativeBool, status: *mut i32) -> DigitalHandle;
    fn HAL_CheckDIOChannel(channel: i32) -> NativeBool;
    fn HAL_FreeDIOPort(handle: DigitalHandle);
    fn HAL_AllocateDigitalPWM(status: *mut i32) -> DigitalPwmHandle;
    fn HAL_FreeDigitalPWM(pwmGenerator: DigitalPwmHandle, status: *mut i32);
    fn HAL_SetDigitalPWMRate(rate: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_SetDigitalPWMDutyCycle(pwmGenerator: DigitalPwmHandle, dutyCycle: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_SetDigitalPWMOutputChannel(pwmGenerator: DigitalPwmHandle, channel: i32, status: *mut i32);
    fn HAL_SetDIO(handle: DigitalHandle, value: NativeBool, status: *mut i32);
    fn HAL_GetDIO(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetDIODirection(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    fn HAL_Pulse(handle: DigitalHandle, pulseLength: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_IsPulsing(handle: DigitalHandle, status: *mut i32) -> NativeBool;
    fn HAL_IsAnyPulsing(status: *mut i32) -> NativeBool;
    fn HAL_SetFilterSelect(handle: DigitalHandle, filterIndex: i32, status: *mut i32);
    fn HAL_GetFilterSelect(handle: DigitalHandle, status: *mut i32) -> i32;
    fn HAL_SetFilterPeriod(filterIndex: i32, value: i64, status: *mut i32);
    fn HAL_GetFilterPeriod(filterIndex: i32, status: *mut i32) -> i64;
}

pub fn initialize_dio_port(handle: PortHandle, input: bool) -> HalResult<DigitalHandle> {
    unsafe { hal_call!(ptr HAL_InitializeDIOPort(handle, input as NativeBool)) }
}

pub fn check_dio_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

pub fn free_dio_port(dio_port_handle: DigitalHandle) {
    unsafe { HAL_FreeDIOPort(dio_port_handle) }
}

pub fn allocate_digital_pwm() -> HalResult<DigitalPwmHandle> {
    unsafe { hal_call!(ptr HAL_AllocateDigitalPWM()) }
}

pub fn free_digital_pwm(pwm_generator: DigitalPwmHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_FreeDigitalPWM(pwm_generator)) }
}

pub fn set_digital_pwm_rate(rate: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetDigitalPWMRate(rate)) }
}

pub fn set_digital_pwm_duty_cycle(pwm_generator: DigitalPwmHandle, duty_cycle: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetDigitalPWMDutyCycle(pwm_generator, duty_cycle)) }
}

pub fn set_digital_pwm_output_channel(pwm_generator: DigitalPwmHandle, channel: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetDigitalPWMOutputChannel(pwm_generator, channel)) }
}

pub fn set_dio(handle: DigitalHandle, value: bool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetDIO(handle, value as NativeBool)) }
}

pub fn get_dio(handle: DigitalHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetDIO(handle)).map(|n| n != 0) }
}

pub fn get_dio_direction(handle: DigitalHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetDIODirection(handle)).map(|n| n != 0) }
}

pub fn pulse(handle: DigitalHandle, pulse_length: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_Pulse(handle, pulse_length)) }
}

pub fn is_pulsing(handle: DigitalHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_IsPulsing(handle)).map(|n| n != 0) }
}

pub fn is_any_pulsing() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_IsAnyPulsing()).map(|n| n != 0) }
}

pub fn set_filter_select(handle: DigitalHandle, filter_index: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetFilterSelect(handle, filter_index)) }
}

pub fn get_filter_select(handle: DigitalHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetFilterSelect(handle)) }
}

pub fn set_filter_period(filter_index: i32, value: i64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetFilterPeriod(filter_index, value)) }
}

pub fn get_filter_period(filter_index: i32) -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetFilterPeriod(filter_index)) }
}
