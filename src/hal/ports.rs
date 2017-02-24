//! Get the amount of hardware on the robot.
//!
//! ## Example
//! ```rust,no_run
//! let how_many_encoders = ports::get_num_encoders();
//! println!("{}", how_many_encoders);
//! ```

#![allow(missing_docs)]

use ::raw::*;

pub fn get_num_accumulators()        -> i32 { unsafe { HAL_GetNumAccumulators() } }
pub fn get_num_analog_triggers()     -> i32 { unsafe { HAL_GetNumAnalogTriggers() } }
pub fn get_num_analog_inputs()       -> i32 { unsafe { HAL_GetNumAnalogInputs() } }
pub fn get_num_analog_outputs()      -> i32 { unsafe { HAL_GetNumAnalogOutputs() } }
pub fn get_num_counters()            -> i32 { unsafe { HAL_GetNumCounters() } }
pub fn get_num_digital_headers()     -> i32 { unsafe { HAL_GetNumDigitalHeaders() } }
pub fn get_num_pwm_headers()         -> i32 { unsafe { HAL_GetNumPWMHeaders() } }
pub fn get_num_digital_channels()    -> i32 { unsafe { HAL_GetNumDigitalChannels() } }
pub fn get_num_pwm_channels()        -> i32 { unsafe { HAL_GetNumPWMChannels() } }
pub fn get_num_digital_pwm_outputs() -> i32 { unsafe { HAL_GetNumDigitalPWMOutputs() } }
pub fn get_num_encoders()            -> i32 { unsafe { HAL_GetNumEncoders() } }
pub fn get_num_interrupts()          -> i32 { unsafe { HAL_GetNumInterrupts() } }
pub fn get_num_relay_channels()      -> i32 { unsafe { HAL_GetNumRelayChannels() } }
pub fn get_num_relay_headers()       -> i32 { unsafe { HAL_GetNumRelayHeaders() } }
pub fn get_num_pcm_modules()         -> i32 { unsafe { HAL_GetNumPCMModules() } }
pub fn get_num_solenoid_channels()   -> i32 { unsafe { HAL_GetNumSolenoidChannels() } }
pub fn get_num_pdp_modules()         -> i32 { unsafe { HAL_GetNumPDPModules() } }
pub fn get_num_pdp_channels()        -> i32 { unsafe { HAL_GetNumPDPChannels() } }
