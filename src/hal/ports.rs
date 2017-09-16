//! Get the amount of hardware on the robot.
//!
//! ## Example
//! ```rust,no_run
//! let how_many_encoders = ports::get_num_encoders();
//! println!("{}", how_many_encoders);
//! ```

extern "C" {
    fn HAL_GetNumAccumulators() -> i32;
    fn HAL_GetNumAnalogTriggers() -> i32;
    fn HAL_GetNumAnalogInputs() -> i32;
    fn HAL_GetNumAnalogOutputs() -> i32;
    fn HAL_GetNumCounters() -> i32;
    fn HAL_GetNumDigitalHeaders() -> i32;
    fn HAL_GetNumPWMHeaders() -> i32;
    fn HAL_GetNumDigitalChannels() -> i32;
    fn HAL_GetNumPWMChannels() -> i32;
    fn HAL_GetNumDigitalPWMOutputs() -> i32;
    fn HAL_GetNumEncoders() -> i32;
    fn HAL_GetNumInterrupts() -> i32;
    fn HAL_GetNumRelayChannels() -> i32;
    fn HAL_GetNumRelayHeaders() -> i32;
    fn HAL_GetNumPCMModules() -> i32;
    fn HAL_GetNumSolenoidChannels() -> i32;
    fn HAL_GetNumPDPModules() -> i32;
    fn HAL_GetNumPDPChannels() -> i32;
}

#[inline(always)] pub fn get_num_accumulators()        -> i32 { unsafe { HAL_GetNumAccumulators() } }
#[inline(always)] pub fn get_num_analog_triggers()     -> i32 { unsafe { HAL_GetNumAnalogTriggers() } }
#[inline(always)] pub fn get_num_analog_inputs()       -> i32 { unsafe { HAL_GetNumAnalogInputs() } }
#[inline(always)] pub fn get_num_analog_outputs()      -> i32 { unsafe { HAL_GetNumAnalogOutputs() } }
#[inline(always)] pub fn get_num_counters()            -> i32 { unsafe { HAL_GetNumCounters() } }
#[inline(always)] pub fn get_num_digital_headers()     -> i32 { unsafe { HAL_GetNumDigitalHeaders() } }
#[inline(always)] pub fn get_num_pwm_headers()         -> i32 { unsafe { HAL_GetNumPWMHeaders() } }
#[inline(always)] pub fn get_num_digital_channels()    -> i32 { unsafe { HAL_GetNumDigitalChannels() } }
#[inline(always)] pub fn get_num_pwm_channels()        -> i32 { unsafe { HAL_GetNumPWMChannels() } }
#[inline(always)] pub fn get_num_digital_pwm_outputs() -> i32 { unsafe { HAL_GetNumDigitalPWMOutputs() } }
#[inline(always)] pub fn get_num_encoders()            -> i32 { unsafe { HAL_GetNumEncoders() } }
#[inline(always)] pub fn get_num_interrupts()          -> i32 { unsafe { HAL_GetNumInterrupts() } }
#[inline(always)] pub fn get_num_relay_channels()      -> i32 { unsafe { HAL_GetNumRelayChannels() } }
#[inline(always)] pub fn get_num_relay_headers()       -> i32 { unsafe { HAL_GetNumRelayHeaders() } }
#[inline(always)] pub fn get_num_pcm_modules()         -> i32 { unsafe { HAL_GetNumPCMModules() } }
#[inline(always)] pub fn get_num_solenoid_channels()   -> i32 { unsafe { HAL_GetNumSolenoidChannels() } }
#[inline(always)] pub fn get_num_pdp_modules()         -> i32 { unsafe { HAL_GetNumPDPModules() } }
#[inline(always)] pub fn get_num_pdp_channels()        -> i32 { unsafe { HAL_GetNumPDPChannels() } }
