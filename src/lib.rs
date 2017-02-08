//! This is a highly experimental crate providing bindings to WPILIB's HAL.
//! This crate is not affiliated with FIRST, FRC, or National Instruments in any way
#![feature(str_escape)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

// #![deny(missing_docs)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate time;

/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

/// Contains bindings to the HAL
pub mod hal;
mod raw;

pub use hal::*;
