use error::*;
use types::*;
use raw::*;
use std::ffi::CStr;
use std::os::raw::c_char;

static mut HAL_INITIALIZED: bool = false;

pub mod types;
pub mod accelerometer;
pub mod analog_gyro;
pub mod analog;
pub mod compressor;
pub mod counter;
pub mod dio;
pub mod driverstation;
pub mod encoder;
pub mod i2c;
pub mod interrupt;
pub mod pdp;
pub mod ports;
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
    pub fn HAL_Initialize(mode: i32) -> i32;
    pub fn HAL_Report(resource: i32, instanceNumber: i32, context: i32, feature: *const c_char) -> i64;
}

pub fn system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}

pub unsafe fn get_error_message(code: i32) -> String {
    let char_ptr = HAL_GetErrorMessage(code);
    CStr::from_ptr(char_ptr).to_string_lossy().into_owned()
}

pub fn get_fpga_version() -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetFPGAVersion()) }
}

pub fn get_fpga_revision() -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetFPGARevision()) }
}

pub fn get_runtime_type() -> RuntimeType {
    unsafe { HAL_GetRuntimeType() }
}

pub fn get_fpga_button() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetFPGAButton()).map(|n| n != 0) }
}

pub fn get_system_active() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetSystemActive()).map(|n| n != 0) }
}

/// Gets whether the robot is underpowered. Basically nothing will work if the bot is browned out.
pub fn get_browned_out() -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetBrownedOut()).map(|n| n != 0) }
}

pub fn base_initialize() -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_BaseInitialize()) }
}

pub fn get_port(channel: i32) -> PortHandle {
    unsafe { HAL_GetPort(channel) }
}

pub fn get_port_with_module(module: i32, channel: i32) -> PortHandle {
    unsafe { HAL_GetPortWithModule(module, channel) }
}

pub fn get_fpga_time() -> HalResult<u64> {
    unsafe { hal_call!(ptr HAL_GetFPGATime()) }
}

pub unsafe fn hal_initialize(mode: i32) -> i32 {
    HAL_INITIALIZED = true;

    HAL_Initialize(mode)
}

pub unsafe fn hal_is_initialized() -> bool {
    HAL_INITIALIZED
}

pub fn report(resource: i32, instance_number: i32, context: i32, feature: &[u8]) -> i64 {
    unsafe { HAL_Report(resource, instance_number, context, feature.as_ptr() as *const c_char) }
}
