use ::error::*;
use ::handle::*;
use ::raw::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;

macro_rules! impl_convert {
    ($a:ident, $b:ident; $($variant_a:ident <=> $variant_b:ident),*) => {
        impl From<$a> for $b {
            fn from(val: $a) -> $b {
                match val {
                    $($a::$variant_a => $b::$variant_b,)*
                }
            }
        }

        impl From<$b> for $a {
            fn from(val: $b) -> $a {
                match val {
                    $($b::$variant_b => $a::$variant_a,)*
                }
            }
        }
    };
}

lazy_static! {
    static ref HAL_INITIALIZED: Mutex<bool> = Mutex::new(false);
}

/// Contains typedefs that rename some raw types with proper names
pub mod handle;

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

pub mod encoder;

/// Bindings to i2c
pub mod i2c;

// TODO
/// What?
pub mod interrupt;

// TODO
// What?
// pub mod notifier;

/// Bindings to the power distribution panel
pub mod pdp;

/// Contains functions that get the number of ports on the bot
pub mod ports;

/// Bindings to power information getters
pub mod power;

/// Bindings to PWMs
pub mod pwm;

/// Bindings to relays
pub mod relay;

/// Aggregation of similar APIs (SPI, I2C, Serial port, OS serial port)
pub mod halio;

/// Bindings to serial ports
pub mod serial;

/// Bindings to solenoids
pub mod solenoid;

/// Bindings to SPI (Serial Port Interface)
pub mod spi;

#[allow(missing_docs)] pub type RawRuntimeType = HAL_RuntimeType;

/// Which environment the robot code is running on.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeType {
    /// Running in an Athena environment
    Native,
    /// Running on a Mock environment
    Mock
}

impl From<RawRuntimeType> for RuntimeType {
    fn from(raw: RawRuntimeType) -> Self {
        match raw {
            HAL_RuntimeType::HAL_Athena => RuntimeType::Native,
            HAL_RuntimeType::HAL_Mock => RuntimeType::Mock,
        }
    }
}


// I think this was in HAL/Constants.h?
/// Gets how many clock ticks occur per microsecond
pub fn get_system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}

pub fn get_error_message(code: i32) -> String {
    unsafe {
        let char_ptr = HAL_GetErrorMessage(code);
        CStr::from_ptr(char_ptr).to_string_lossy().into_owned()
    }
}

pub fn get_fpga_version() -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetFPGAVersion() ] }
}

pub fn get_fpga_revision() -> HalResult<i64> {
    unsafe { hal_call![ ptr HAL_GetFPGARevision() ] }
}

pub fn get_runtime_type() -> RuntimeType {
    unsafe { RuntimeType::from(HAL_GetRuntimeType()) }
}

pub fn get_fpga_button() -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetFPGAButton() ].map(|n| n != 0) }
}

pub fn get_system_active() -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetSystemActive() ].map(|n| n != 0) }
}

/// Gets whether the robot is underpowered. Basically nothing will work if the bot is browned out.
pub fn get_browned_out() -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetBrownedOut() ].map(|n| n != 0) }
}

pub fn base_initialize() -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_BaseInitialize() ] }
}

pub fn get_port(channel: i32) -> PortHandle {
    unsafe { HAL_GetPort(channel) }
}

pub fn get_port_with_module(module: i32, channel: i32) -> PortHandle {
    unsafe { HAL_GetPortWithModule(module, channel) }
}

pub fn get_fpga_time() -> HalResult<u64> {
    unsafe { hal_call![ ptr HAL_GetFPGATime() ] }
}

/// Initialize the HAL
pub fn hal_initialize(mode: i32) -> i32 {
    let mut initialized = HAL_INITIALIZED.lock().unwrap();
    *initialized = true;

    unsafe { HAL_Initialize(mode) }
}

pub fn hal_is_initialized() -> bool {
    // Our code will never panic, so we can just unwrap the lock
    // The value is copied and the lock is dropped before the end of the function
    *HAL_INITIALIZED.lock().unwrap()
}

pub fn report(resource: i32, instance_number: i32, context: i32, feature: &[u8]) -> i64 {
    unsafe {
        HAL_Report(resource,
                   instance_number,
                   context,
                   feature.as_ptr() as *const c_char)
    }
}
