use ::error::*;
use hal::handle::*;
use ::raw::*;

pub unsafe fn initialize_dio_port(handle: PortHandle, input: bool) -> HalResult<DigitalHandle> {
    hal_call![ ptr HAL_InitializeDIOPort(handle, input as HAL_Bool) ]
}

pub unsafe fn check_dio_channel(channel: i32) -> bool {
    HAL_CheckDIOChannel(channel) != 0
}

pub unsafe fn free_dio_port(dio_port_handle: DigitalHandle) {
    HAL_FreeDIOPort(dio_port_handle)
}

pub unsafe fn allocate_digital_pwm() -> HalResult<DigitalPwmHandle> {
    hal_call![ ptr HAL_AllocateDigitalPWM() ]
}

pub unsafe fn free_digital_pwm(pwm_generator: DigitalPwmHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreeDigitalPWM(pwm_generator) ]
}

pub unsafe fn set_digital_pwm_rate(rate: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMRate(rate) ]
}

pub unsafe fn set_digital_pwm_duty_cycle(pwm_generator: DigitalPwmHandle, duty_cycle: f64)
                                  -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMDutyCycle(pwm_generator, duty_cycle) ]
}

pub unsafe fn set_digital_pwm_output_channel(pwm_generator: DigitalPwmHandle, channel: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetDigitalPWMOutputChannel(pwm_generator, channel) ]
}

pub unsafe fn set_dio(handle: DigitalHandle, value: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetDIO(handle, value as HAL_Bool) ]
}

pub unsafe fn get_dio(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetDIO(handle) ].map(|n| n != 0)
}

pub unsafe fn get_dio_direction(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetDIODirection(handle) ].map(|n| n != 0)
}

pub unsafe fn pulse(handle: DigitalHandle, pulse_length: f64) -> HalResult<()> {
    hal_call![ ptr HAL_Pulse(handle, pulse_length) ]
}

pub unsafe fn is_pulsing(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_IsPulsing(handle) ].map(|n| n != 0)
}

pub unsafe fn is_any_pulsing() -> HalResult<bool> {
    hal_call![ ptr HAL_IsAnyPulsing() ].map(|n| n != 0)
}

pub unsafe fn set_filter_select(handle: DigitalHandle, filter_index: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetFilterSelect(handle, filter_index) ]
}

pub unsafe fn get_filter_select(handle: DigitalHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetFilterSelect(handle) ]
}

pub unsafe fn set_filter_period(filter_index: i32, value: i64) -> HalResult<()> {
    hal_call![ ptr HAL_SetFilterPeriod(filter_index, value) ]
}

pub unsafe fn get_filter_period(filter_index: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetFilterPeriod(filter_index) ]
}
