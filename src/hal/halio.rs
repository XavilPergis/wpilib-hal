//! Higher-order functions to abstract over different IO mechanisms on the RoboRIO
//!
//! ## Example
//! ```rust,no_run
//! let mut data = b"Hello world!";
//! halio::io_write(i2c::write, RobotIoPort::I2c(), &data, data.len())
//! ```

use ::error::*;
use serial::SerialPort;
use spi::SpiPort;

pub enum RobotIoPort {
    Serial(SerialPort),
    Spi(SpiPort),
    /// (port, address)
    I2c(i32, i32)
}

pub fn io_read<F>(func: F, port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32>
    where F: Fn(RobotIoPort, &mut [u8], i32) -> HalResult<i32> {
    func(port, buffer, count)
}

pub fn io_write<F>(func: F, port: RobotIoPort, buffer: &[u8], count: i32) -> HalResult<i32>
    where F: Fn(RobotIoPort, &[u8], i32) -> HalResult<i32> {
    func(port, buffer, count)
}

pub fn io_clear<F>(func: F, port: RobotIoPort) -> HalResult<()>
    where F: Fn(RobotIoPort) -> HalResult<()> {
    func(port)
}
