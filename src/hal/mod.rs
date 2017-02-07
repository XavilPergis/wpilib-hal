use ::raw::*;

/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

// TODO: Separate out
pub mod enums;

// TODO: Seperate out
pub mod structs;

/// Bindings for the on-board accelerometer
pub mod accelerometer;

/// Bindings for analog accumulators
pub mod accumulator;

/// Bindings for analog gyros
pub mod analog_gyro;

/// Bindings for analog inputs
pub mod analog_input;

/// Bindings for analog outputs
pub mod analog_output;

/// Bindings for analog triggers
pub mod analog_trigger;

/// Bindings for CAN
pub mod can;

/// Bindings for compressors
pub mod compressor;

// TODO
/// Bindings for ???
pub mod counter;

/// Bindings to Digital IO
pub mod dio;

/// Bindings to the driver station
pub mod driverstation;

/// Bindings to i2c
pub mod i2c;

// TODO
/// What?
pub mod interrupt;

// TODO
/// What?
pub mod notifier;

/// Bindings to the power distribution panel
pub mod pdp;

/// Contains functions that get the number of ports on the bot
pub mod ports;

/// Bindings to power information getters
pub mod power;

/// Bindings to pwms
pub mod pwm;

/// Bindings to relays
pub mod relay;

/// Aggregation of similar APIs (SPI, I2C, Serial port, OS serial port)
pub mod serial_io;

/// Bindings to serial ports
pub mod serial;

/// Bindings to solenoids
pub mod solenoid;

/// Bindings to SPI (Serial Port Interface)
pub mod spi;

// I think this was in HAL/Constants.h?
pub fn get_system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}

pub fn get_error_message(code: i32) -> String {
    let char_ptr = unsafe { HAL_GetErrorMessage(code) };
    CStr::from_ptr(char_ptr).to_string_lossy().into_owned()
}

pub fn get_fpga_version() -> HalResult<i32> {
    hal_call![ ptr HAL_GetFPGAVersion() ]
}

pub fn get_fpga_revision() -> HalResult<i64> {
    hal_call![ ptr HAL_GetFPGARevision() ]
}

pub fn get_runtime_type() -> RuntimeType {
    RuntimeType::from(unsafe { HAL_GetRuntimeType() })
}

pub fn get_fpga_button() -> HalResult<bool> {
    hal_call![ ptr HAL_GetFPGAButton() ].map(|n| n != 0)
}

pub fn get_system_active() -> HalResult<bool> {
    hal_call![ ptr HAL_GetSystemActive() ].map(|n| n != 0)
}

pub fn get_browned_out() -> HalResult<bool> {
    hal_call![ ptr HAL_GetBrownedOut() ].map(|n| n != 0)
}

pub fn base_initialize() {
    unsafe { HAL_BaseInitialize() }
}

pub fn get_port(channel: i32) -> PortHandle {
    PortHandle(unsafe { HAL_GetPort(channel) })
}

pub fn get_port_with_module(module: i32, channel: i32) -> PortHandle {
    PortHandle(unsafe { HAL_GetPortWithModule(module, channel) })
}

pub fn get_fpga_time() -> HalResult<u64> {
    hal_call![ ptr HAL_GetFPGATime() ]
}

/// Initialize the HAL
pub fn hal_initialize(mode: i32) -> i32 {
    unsafe { HAL_Initialize(mode) }
}

pub fn report(resource: i32, instance_number: i32, context: i32, feature: &[u8]) -> i64 {
    unsafe { HAL_Report(resource, instance_number, context, feature.as_ptr()) }
}
