use ::error::*;
use hal::handle::*;
use ::raw::*;
use std::mem;

pub struct PwmConfig {
    max_pwm: i32,
    deadband_max_pwm: i32,
    center_pwm: i32,
    deadband_min_pwm: i32,
    min_pwm: i32,
}

pub fn initialize(handle: PortHandle) -> HalResult<DigitalHandle> {
    unsafe { hal_call![ ptr HAL_InitializePWMPort(handle) ] }
}

pub fn free(handle: DigitalHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_FreePWMPort(handle) ] }
}

pub fn check_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPWMChannel(channel) != 0 }
}

pub fn set_config(handle: DigitalHandle, max_pwm: f64, deadband_max_pwm: f64, center_pwm: f64, deadband_min_pwm: f64, min_pwm: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMConfig(handle, max_pwm, deadband_max_pwm, center_pwm, deadband_min_pwm, min_pwm) ] }
}

pub fn set_config_raw(handle: DigitalHandle, cfg: PwmConfig) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMConfigRaw(handle, cfg.max_pwm, cfg.deadband_max_pwm, cfg.center_pwm, cfg.deadband_min_pwm, cfg.min_pwm) ] }
}

pub fn get_config_raw(handle: DigitalHandle) -> HalResult<PwmConfig> {
    // Create a zeroed struct. Will either be filled, or an Err will be returned
    // and cfg will be dropped
    let mut cfg = unsafe {
        PwmConfig {
            max_pwm: mem::zeroed(),
            deadband_max_pwm: mem::zeroed(),
            center_pwm: mem::zeroed(),
            deadband_min_pwm: mem::zeroed(),
            min_pwm: mem::zeroed(),
        }
    };

    // &mut T can be coerced to *mut T
    unsafe { hal_call![ ptr HAL_GetPWMConfigRaw(
        handle,
        &mut cfg.max_pwm,
        &mut cfg.deadband_max_pwm,
        &mut cfg.center_pwm,
        &mut cfg.deadband_min_pwm,
        &mut cfg.min_pwm
    ) ]?; }

    Ok(cfg)
}

pub fn set_eliminate_deadband(handle: DigitalHandle, eliminate_deadband: bool)
                                  -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMEliminateDeadband(handle, eliminate_deadband as HAL_Bool) ] }
}

pub fn get_eliminate_deadband(handle: DigitalHandle) -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetPWMEliminateDeadband(handle) ].map(|n| n != 0) }
}

pub fn set_raw(handle: DigitalHandle, value: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMRaw(handle, value) ] }
}

pub fn set_speed(handle: DigitalHandle, speed: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMSpeed(handle, speed) ] }
}

pub fn set_position(handle: DigitalHandle, position: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMPosition(handle, position) ] }
}

pub fn set_disabled(handle: DigitalHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMDisabled(handle) ] }
}

pub fn get_raw(handle: DigitalHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetPWMRaw(handle) ] }
}

pub fn get_speed(handle: DigitalHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetPWMSpeed(handle) ] }
}

pub fn get_position(handle: DigitalHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetPWMPosition(handle) ] }
}

pub fn latch_zero(handle: DigitalHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_LatchPWMZero(handle) ] }
}

pub fn set_period_scale(handle: DigitalHandle, squelch_mask: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetPWMPeriodScale(handle, squelch_mask) ] }
}

pub fn get_loop_timing() -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetLoopTiming() ] }
}
