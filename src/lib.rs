//! This is a highly experimental crate providing bindings to WPILIB's HAL.
//! This crate is not affiliated with FIRST, FRC, or National Instruments in
//! any way
#![feature(custom_attribute)]

// Enable clippy linting
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

// Only deny missing docs in release mode
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![cfg_attr(debug_assertions, allow(missing_docs))]
#![allow(dead_code)]

extern crate time;

/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

#[macro_use]
pub mod log;

/// Contains bindings to the HAL
#[macro_use]
pub mod hal;

#[allow(non_camel_case_types, non_snake_case)]
mod raw;

pub use hal::*;
