#![allow(missing_docs)]

use ::raw::*;

/// Get the amount of hardware on the robot.
///
/// ## Example
/// ```rust,no_run
/// let how_many_encoders = ports::get_num(ports::encoders);
/// println!("{}", how_many_encoders);
/// ```
pub fn get_num<F>(func: F) -> i32 where F: Fn() -> i32 { func() }

pub fn accumulators()        -> i32 { unsafe { HAL_GetNumAccumulators() } }
pub fn analog_triggers()     -> i32 { unsafe { HAL_GetNumAnalogTriggers() } }
pub fn analog_inputs()       -> i32 { unsafe { HAL_GetNumAnalogInputs() } }
pub fn analog_outputs()      -> i32 { unsafe { HAL_GetNumAnalogOutputs() } }
pub fn counters()            -> i32 { unsafe { HAL_GetNumCounters() } }
pub fn digital_headers()     -> i32 { unsafe { HAL_GetNumDigitalHeaders() } }
pub fn pwm_headers()         -> i32 { unsafe { HAL_GetNumPWMHeaders() } }
pub fn digital_channels()    -> i32 { unsafe { HAL_GetNumDigitalChannels() } }
pub fn pwm_channels()        -> i32 { unsafe { HAL_GetNumPWMChannels() } }
pub fn digital_pwm_outputs() -> i32 { unsafe { HAL_GetNumDigitalPWMOutputs() } }
pub fn encoders()            -> i32 { unsafe { HAL_GetNumEncoders() } }
pub fn interrupts()          -> i32 { unsafe { HAL_GetNumInterrupts() } }
pub fn relay_channels()      -> i32 { unsafe { HAL_GetNumRelayChannels() } }
pub fn relay_headers()       -> i32 { unsafe { HAL_GetNumRelayHeaders() } }
pub fn pcm_modules()         -> i32 { unsafe { HAL_GetNumPCMModules() } }
pub fn solenoid_channels()   -> i32 { unsafe { HAL_GetNumSolenoidChannels() } }
pub fn pdp_modules()         -> i32 { unsafe { HAL_GetNumPDPModules() } }
pub fn pdp_channels()        -> i32 { unsafe { HAL_GetNumPDPChannels() } }
