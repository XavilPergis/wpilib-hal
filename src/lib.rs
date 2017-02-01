//! This is a highly experimental crate providing bindings to WPILIB's HAL.
//! This crate is not affiliated with FIRST, FRC, or National Instruments in any way
#![feature(str_escape)]

#![deny(missing_docs)]
#![allow(dead_code)]

extern crate time;

#[macro_use]
pub mod hal;
pub mod raw;

pub use hal::*;
