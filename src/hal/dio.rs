use ::raw::*;
use hal::handle::*;
use ::error::*;

pub fn initialize_dio_port(handle: PortHandle, input: bool) -> HalResult<DigitalHandle> {
    hal_call![ ptr HAL_InitializeDIOPort(handle.get_handle(), input as HAL_Bool) ]
        .map(|n| DigitalHandle(n))
}

pub fn check_dio_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

pub fn free_dio_port(dio_port_handle: DigitalHandle) {
    unsafe { HAL_FreeDIOPort(dio_port_handle.get_handle()) }
}

pub fn allocate_digital_pwm() -> HalResult<DigitalPwmHandle> {
    hal_call![ ptr HAL_AllocateDigitalPWM() ].map(|n| DigitalPwmHandle(n))
}

pub fn free_digital_pwm(pwm_generator: DigitalPwmHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreeDigitalPWM(pwm_generator.get_handle()) ]
}

pub fn set_digital_pwm_rate(rate: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMRate(rate) ]
}

pub fn set_digital_pwm_duty_cycle(pwm_generator: DigitalPwmHandle,
                                  duty_cycle: f64)
                                  -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMDutyCycle(pwm_generator.get_handle(), duty_cycle) ]
}

pub fn set_digital_pwm_output_channel(pwm_generator: DigitalPwmHandle,
                                      channel: i32)
                                      -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMOutputChannel(pwm_generator.get_handle(), channel) ]
}

pub fn set_dio(handle: DigitalHandle, value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetDIO(handle.get_handle(), value as HAL_Bool) ]
}

pub fn get_dio(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetDIO(handle.get_handle()) ].map(|n| n != 0)
}

pub fn get_dio_direction(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetDIODirection(handle.get_handle()) ].map(|n| n != 0)
}

pub fn pulse(handle: DigitalHandle, pulse_length: f64) -> HalResult<()> {
    hal_call![ ptr HAL_Pulse(handle.get_handle(), pulse_length) ]
}

pub fn is_pulsing(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_IsPulsing(handle.get_handle()) ].map(|n| n != 0)
}

pub fn is_any_pulsing() -> HalResult<bool> {
    hal_call![ ptr HAL_IsAnyPulsing() ].map(|n| n != 0)
}

pub fn set_filter_select(handle: DigitalHandle, filter_index: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetFilterSelect(handle.get_handle(), filter_index) ]
}

pub fn get_filter_select(handle: DigitalHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetFilterSelect(handle.get_handle()) ]
}

pub fn set_filter_period(filter_index: i32, value: i64) -> HalResult<()> {
    hal_call![ ptr HAL_SetFilterPeriod(filter_index, value) ]
}

pub fn get_filter_period(filter_index: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetFilterPeriod(filter_index) ]
}
