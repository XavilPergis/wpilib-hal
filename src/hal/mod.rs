/// Contains aggregate error types and macros for calling FFI functions
#[macro_use]
pub mod error;

pub mod driverstation;
pub mod wrapper;

// TODO: Separate out
pub mod enums;

// TODO: Seperate out
pub mod structs;

pub mod spi;
pub mod serial;
pub mod i2c;
