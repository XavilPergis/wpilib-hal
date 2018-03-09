//! This is a highly experimental crate providing bindings to WPILIB's HAL.
//! This crate is not affiliated with FIRST, FRC, or National Instruments in
//! any way

// Enable clippy linting
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![deny(missing_debug_implementations)]

extern crate backtrace;

macro_rules! require_value_between {
    ($val:expr, $low:expr, $high:expr) => {
        if $val > $high || $val < $low {
            panic!("Value out of bounds. Range is {} to {}, but the actual value was {}", $low, $high, $val);
        }
    };
}

/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

/// Contains bindings to the HAL
#[macro_use]
pub mod hal;
pub use hal::*;
