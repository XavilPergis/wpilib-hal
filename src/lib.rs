//! This is a highly experimental crate providing bindings to WPILIB's HAL.
//! This crate is not affiliated with FIRST, FRC, or National Instruments in
//! any way
#![feature(str_escape, custom_attribute, field_init_shorthand)]

// Enable clippy linting
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

// Only deny missing docs in release mode
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![cfg_attr(debug_assertions, allow(missing_docs))]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate time;

/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

/// Contains bindings to the HAL
pub mod hal;

#[allow(non_camel_case_types, non_snake_case)]
mod raw;

pub use hal::*;
