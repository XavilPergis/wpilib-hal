use ::raw::*;

pub fn initialize_pwm_port(handle: PortHandle) -> HalResult<DigitalHandle> {
    hal_call![ ptr HAL_InitializePWMPort(handle.get_handle()) ].map(|n| DigitalHandle(n))
}

pub fn free_pwm_port(handle: DigitalHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreePWMPort(handle.get_handle()) ]
}

pub fn check_pwm_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPWMChannel(channel) != 0 }
}

pub struct PwmConfig {
    max_pwm: i32,
    deadband_max_pwm: i32,
    center_pwm: i32,
    deadband_min_pwm: i32,
    min_pwm: i32
}

pub fn set_pwm_config(handle: DigitalHandle, cfg: PwmConfig) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMConfig(handle.get_handle(), cfg.max_pwm, cfg.deadband_max_pwm, cfg.center_pwm, cfg.deadband_min_pwm, cfg.min_pwm) ]
}

pub fn set_pwm_config_raw(handle: DigitalHandle, cfg: PwmConfig) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMConfigRaw(handle.get_handle(), cfg.max_pwm, cfg.deadband_max_pwm, cfg.center_pwm, cfg.deadband_min_pwm, cfg.min_pwm) ]
}

pub fn get_pwm_config_raw(handle: DigitalHandle) -> HalResult<PwmConfig> {
    use std::mem;

    // Create a zeroed struct. Will either be filled, or an Err will be returned and cfg will be dropped
    let cfg = PwmConfig {
        max_pwm: mem::zeroed(),
        deadband_max_pwm: mem::zeroed(),
        center_pwm: mem::zeroed(),
        deadband_min_pwm: mem::zeroed(),
        min_pwm: mem::zeroed()
    };

    hal_call![ ptr HAL_GetPWMConfigRaw(
    handle.get_handle(),
    &mut cfg.max_pwm as *mut i32,
    &mut cfg.deadband_max_pwm as *mut i32,
    &mut cfg.center_pwm as *mut i32,
    &mut cfg.deadband_min_pwm as *mut i32,
    &mut cfg.min_pwm as *mut i32
) ]
}

pub fn set_pwm_eliminate_deadband(handle: DigitalHandle, eliminate_deadband: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMEliminateDeadband(handle.get_handle(), eliminate_deadband as HAL_Bool) ]
}

pub fn get_pwm_eliminate_deadband(handle: DigitalHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetPWMEliminateDeadband(handle.get_handle()) ].map(|n| n != 0)
}

pub fn set_pwm_raw(handle: DigitalHandle, value: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMRaw(handle.get_handle(), value) ]
}

pub fn set_pwm_speed(handle: DigitalHandle, speed: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMSpeed(handle.get_handle(), speed) ]
}

pub fn set_pwm_position(handle: DigitalHandle, position: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMPosition(handle.get_handle(), position) ]
}

pub fn set_pwm_disabled(handle: DigitalHandle) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMDisabled(handle.get_handle()) ]
}

pub fn get_pwm_raw(handle: DigitalHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetPWMRaw(handle.get_handle()) ]
}

pub fn get_pwm_speed(handle: DigitalHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPWMSpeed(handle.get_handle()) ]
}

pub fn get_pwm_position(handle: DigitalHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetPWMPosition(handle.get_handle()) ]
}

pub fn latch_pwm_zero(handle: DigitalHandle) -> HalResult<()> {
    hal_call![ ptr HAL_LatchPWMZero(handle.get_handle()) ]
}

pub fn set_pwm_period_scale(handle: DigitalHandle, squelch_mask: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetPWMPeriodScale(handle.get_handle(), squelch_mask) ]
}

pub fn get_loop_timing() -> HalResult<i32> {
    hal_call![ ptr HAL_GetLoopTiming() ]
}
