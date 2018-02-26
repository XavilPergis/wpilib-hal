use error::*;
use types::*;
use raw::*;
use std::ffi::CStr;
use std::os::raw::c_char;

pub mod types;
pub mod accelerometer;
pub mod analog_gyro;
pub mod analog_trigger;
pub mod analog;
pub mod compressor;
pub mod counter;
pub mod dio;
pub mod driverstation;
pub mod encoder;
pub mod i2c;
pub mod interrupt;
pub mod pdp;
pub mod power;
pub mod pwm;
pub mod relay;
pub mod serial;
pub mod solenoid;
pub mod spi;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeType {
    Native = 0,
    Mock = 1,
}

extern "C" {
    pub fn HAL_GetErrorMessage(code: i32) -> *const c_char;
    pub fn HAL_GetFPGAVersion(status: *mut i32) -> i32;
    pub fn HAL_GetFPGARevision(status: *mut i32) -> i64;
    pub fn HAL_GetRuntimeType() -> RuntimeType;
    pub fn HAL_GetFPGAButton(status: *mut i32) -> NativeBool;
    pub fn HAL_GetSystemActive(status: *mut i32) -> NativeBool;
    pub fn HAL_GetBrownedOut(status: *mut i32) -> NativeBool;
    pub fn HAL_BaseInitialize(status: *mut i32);
    pub fn HAL_GetPort(channel: i32) -> PortHandle;
    pub fn HAL_GetPortWithModule(module: i32, channel: i32) -> PortHandle;
    pub fn HAL_GetFPGATime(status: *mut i32) -> u64;
    pub fn HAL_Initialize(timeout: i32, mode: i32) -> NativeBool;
    pub fn HAL_Report(resource: i32, instanceNumber: i32, context: i32, feature: *const c_char) -> i64;
}

macro_rules! num_ports {
    ($($name:ident : $hal_func:ident,)*) => {
        $(
            extern "C" { fn $hal_func() -> i32; }
            #[inline(always)] pub fn $name() -> i32 { unsafe { $hal_func() } }
        )*
    }
}

num_ports! {
    get_num_accumulators: HAL_GetNumAccumulators,
    get_num_analog_triggers: HAL_GetNumAnalogTriggers,
    get_num_analog_inputs: HAL_GetNumAnalogInputs,
    get_num_analog_outputs: HAL_GetNumAnalogOutputs,
    get_num_counters: HAL_GetNumCounters,
    get_num_digital_headers: HAL_GetNumDigitalHeaders,
    get_num_pwm_headers: HAL_GetNumPWMHeaders,
    get_num_digital_channels: HAL_GetNumDigitalChannels,
    get_num_pwm_channels: HAL_GetNumPWMChannels,
    get_num_digital_pwm_outputs: HAL_GetNumDigitalPWMOutputs,
    get_num_encoders: HAL_GetNumEncoders,
    get_num_interrupts: HAL_GetNumInterrupts,
    get_num_relay_channels: HAL_GetNumRelayChannels,
    get_num_relay_headers: HAL_GetNumRelayHeaders,
    get_num_pcm_modules: HAL_GetNumPCMModules,
    get_num_solenoid_channels: HAL_GetNumSolenoidChannels,
    get_num_pdp_modules: HAL_GetNumPDPModules,
    get_num_pdp_channels: HAL_GetNumPDPChannels,
}

#[inline(always)]
pub fn system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}

pub fn get_error_message(code: i32) -> &'static str {
    unsafe {
        // The error messages returned all seem to be static string references, see
        // https://github.com/wpilibsuite/allwpilib/blob/master/hal/src/main/native/athena/HAL.cpp#L97
        // NOTE: all values of `code` are covered by the HAL
        let cstr = CStr::from_ptr::<'static>(HAL_GetErrorMessage(code) as *const _);
        // I'm just going to assume that the string constants from the HAL are valid utf8
        cstr.to_str().unwrap()
    }
}

pub fn get_fpga_version() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetFPGAVersion()) }
}

pub fn get_fpga_revision() -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetFPGARevision()) }
}

/// Get the runtime type; either athena if running on an actual RoboRIO, or mock if running
/// inside a simulation.
pub fn get_runtime_type() -> RuntimeType {
    unsafe { HAL_GetRuntimeType() }
}

/// Get the status of the RoboRIO user button.
pub fn get_fpga_button() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetFPGAButton()).map(|n| n != 0) }
}

pub fn get_system_active() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetSystemActive()).map(|n| n != 0) }
}

/// Get whether the robot is underpowered.
pub fn get_browned_out() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetBrownedOut()).map(|n| n != 0) }
}

/// You could call this, but why would you want to?
pub fn base_initialize() -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_BaseInitialize()) }
}

const INVALID_HANDLE: i32 = 0;

fn check_invalid(val: i32) -> Option<i32> {
    match val {
        INVALID_HANDLE => None,
        v => Some(v),
    }
}

pub fn get_port(channel: i32) -> Option<PortHandle> {
    unsafe { check_invalid(HAL_GetPort(channel)) }
}

pub fn get_port_with_module(module: i32, channel: i32) -> Option<PortHandle> {
    unsafe { check_invalid(HAL_GetPortWithModule(module, channel)) }
}

pub fn get_fpga_time() -> HalResult<u64> {
    unsafe { hal_call!(ptr HAL_GetFPGATime()) }
}

/// Initialize the HAL. Threadsafe.
///
/// This procedure will try to send SIGTERM to the previous process, and then sleep for
/// `timeout` milliseconds. If the program hasn't died by then, one of the following actions
/// will be taken depending on what `mode` is set to:
/// - Mode of `0` will abort the new process if the old one didn't die
/// - Mode of `1` will send SIGKILL to the old program
/// - Any other value will warn that the old process didn't die, but will still launch the new one
pub fn hal_initialize(timeout: i32, mode: i32) -> bool {
    unsafe { HAL_Initialize(timeout, mode) != 0 }
}

pub fn report(resource: i32, instance_number: i32, context: i32, feature: &[u8]) -> i64 {
    unsafe { HAL_Report(resource, instance_number, context, feature.as_ptr() as *const c_char) }
}
