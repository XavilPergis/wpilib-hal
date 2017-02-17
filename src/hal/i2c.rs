use ::error::*;
use ::raw::*;

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
    pub fn get_port(&self) -> i32 {
        match *self {
            I2cPort::OnBoard => 0,
            I2cPort::MXP => 1,
        }
    }
}

fn initialize_i2c(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeI2C(port) ] }
}

// TODO
// fn transaction_i2c(port: i32, device_address: i32, dataToSend: *mut u8,
// sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32 {
//
//

fn write_i2c(port: i32, device_address: i32, data_to_send: &[u8], send_size: i32) -> i32 {
    unsafe {
        HAL_WriteI2C(port, device_address, data_to_send.as_ptr() as *mut u8, send_size)
    }
}

fn read_i2c(port: i32, device_address: i32, buffer: &mut [u8], count: i32) -> i32 {
    unsafe { HAL_ReadI2C(port, device_address, buffer.as_mut_ptr(), count) }
}

fn close_i2c(port: i32) {
    unsafe { HAL_CloseI2C(port) }
}
