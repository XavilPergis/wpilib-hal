use ::error::*;
use ::raw::*;
use serial_io::RobotIoPort;

lazy_static! {
    static ref INITIALIZED_I2C_PORTS: Vec<i32> = Vec::new();
}

/// Which port the I2C device is on
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum I2cPort {
    OnBoard,
    MXP,
}

impl I2cPort {
    /// Get the integer representation of the port
    pub fn get_port(&self) -> i32 {
        match *self {
            I2cPort::OnBoard => 0,
            I2cPort::MXP => 1,
        }
    }
}

pub fn initialize_i2c(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeI2C(port) ] }
}

// TODO
// pub fn transaction_i2c(port: i32, device_address: i32, dataToSend: *mut u8,
// sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32 {
//
//

/// Write a buffer to the I2C
pub fn write_i2c(io_port: RobotIoPort, data_to_send: &[u8], send_size: i32) -> HalResult<i32> {
    if let RobotIoPort::I2c(port, address) = io_port {
        unsafe { Ok(HAL_WriteI2C(port, address, data_to_send.as_ptr() as *mut u8, send_size)) }
    } else {
        Err(HalError::WrongIoInterface)
    }
}

/// Read a buffer of data from the I2C
pub fn read_i2c(io_port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    if let RobotIoPort::I2c(port, address) = io_port {
        unsafe { Ok(HAL_ReadI2C(port, address, buffer.as_mut_ptr(), count)) }
    } else {
        Err(HalError::WrongIoInterface)
    }
}

/// Close an I2C interface
pub fn close_i2c(io_port: RobotIoPort) -> HalResult<()> {
    if let RobotIoPort::I2c(port, _) = io_port {
        unsafe { Ok(HAL_CloseI2C(port)) }
    } else {
        Err(HalError::WrongIoInterface)
    }
}
