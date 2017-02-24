use ::error::*;
use ::raw::*;
use halio::RobotIoPort;

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

pub fn initialize(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeI2C(port) ] }
}

// TODO
pub fn transaction(io_port: RobotIoPort, send_buffer: &[u8], send_size: i32, recv_buffer: &mut [u8], recv_size: i32) -> HalResult<i32> {
    let (port, address) = io_port.as_i2c()?;
    unsafe {
        Ok(HAL_TransactionI2C(port, address, send_buffer.as_ptr() as *mut u8, send_size, recv_buffer.as_mut_ptr(), recv_size))
    }
}

/// Write a buffer to the I2C
pub fn write(io_port: RobotIoPort, data_to_send: &[u8], send_size: i32) -> HalResult<i32> {
    let (port, address) = io_port.as_i2c()?;
    unsafe { Ok(HAL_WriteI2C(port, address, data_to_send.as_ptr() as *mut u8, send_size)) }
}

/// Read a buffer of data from the I2C
pub fn read(io_port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    let (port, address) = io_port.as_i2c()?;
    unsafe { Ok(HAL_ReadI2C(port, address, buffer.as_mut_ptr(), count)) }
}

/// Close an I2C interface
pub fn close(io_port: RobotIoPort) -> HalResult<()> {
    unsafe { Ok(HAL_CloseI2C(io_port.as_i2c()?.0)) }
}
