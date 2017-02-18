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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RobotIoPort {
    Serial(SerialPort),
    Spi(SpiPort),
    /// (port, address)
    I2c(i32, i32)
}

impl RobotIoPort {
    pub fn as_serial(&self) -> HalResult<SerialPort> {
        if let RobotIoPort::Serial(port) = *self {
            Ok(port)
        } else {
            Err(HalError::WrongIoInterface)
        }
    }

    pub fn as_spi(&self) -> HalResult<SpiPort> {
        if let RobotIoPort::Spi(port) = *self {
            Ok(port)
        } else {
            Err(HalError::WrongIoInterface)
        }
    }

    pub fn as_i2c(&self) -> HalResult<(i32, i32)> {
        if let RobotIoPort::I2c(port, addr) = *self {
            Ok((port, addr))
        } else {
            Err(HalError::WrongIoInterface)
        }
    }
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
