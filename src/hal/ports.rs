//! Get the amount of hardware on the robot.
//!
//! ## Example
//! ```rust,no_run
//! let how_many_encoders = ports::get_num_encoders();
//! println!("{}", how_many_encoders);
//! ```

#![allow(missing_docs)]

use ::raw::*;

pub unsafe fn get_num_accumulators()        -> i32 { HAL_GetNumAccumulators() }
pub unsafe fn get_num_analog_triggers()     -> i32 { HAL_GetNumAnalogTriggers() }
pub unsafe fn get_num_analog_inputs()       -> i32 { HAL_GetNumAnalogInputs() }
pub unsafe fn get_num_analog_outputs()      -> i32 { HAL_GetNumAnalogOutputs() }
pub unsafe fn get_num_counters()            -> i32 { HAL_GetNumCounters() }
pub unsafe fn get_num_digital_headers()     -> i32 { HAL_GetNumDigitalHeaders() }
pub unsafe fn get_num_pwm_headers()         -> i32 { HAL_GetNumPWMHeaders() }
pub unsafe fn get_num_digital_channels()    -> i32 { HAL_GetNumDigitalChannels() }
pub unsafe fn get_num_pwm_channels()        -> i32 { HAL_GetNumPWMChannels() }
pub unsafe fn get_num_digital_pwm_outputs() -> i32 { HAL_GetNumDigitalPWMOutputs() }
pub unsafe fn get_num_encoders()            -> i32 { HAL_GetNumEncoders() }
pub unsafe fn get_num_interrupts()          -> i32 { HAL_GetNumInterrupts() }
pub unsafe fn get_num_relay_channels()      -> i32 { HAL_GetNumRelayChannels() }
pub unsafe fn get_num_relay_headers()       -> i32 { HAL_GetNumRelayHeaders() }
pub unsafe fn get_num_pcm_modules()         -> i32 { HAL_GetNumPCMModules() }
pub unsafe fn get_num_solenoid_channels()   -> i32 { HAL_GetNumSolenoidChannels() }
pub unsafe fn get_num_pdp_modules()         -> i32 { HAL_GetNumPDPModules() }
pub unsafe fn get_num_pdp_channels()        -> i32 { HAL_GetNumPDPChannels() }
