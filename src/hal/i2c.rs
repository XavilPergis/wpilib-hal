use error::*;

// NOTE: all the `*const u8`s here are actually `*mut u8` but are immediately cast to a const pointer in the implementation
extern "C" {
    pub fn HAL_InitializeI2C(port: I2cPort, status: *mut i32);
    pub fn HAL_TransactionI2C(port: I2cPort, deviceAddress: i32, dataToSend: *const u8, sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32;
    pub fn HAL_WriteI2C(port: I2cPort, deviceAddress: i32, dataToSend: *const u8, sendSize: i32) -> i32;
    pub fn HAL_ReadI2C(port: I2cPort, deviceAddress: i32, buffer: *mut u8, count: i32) -> i32;
    pub fn HAL_CloseI2C(port: I2cPort);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum I2cPort {
    OnBoard = 0,
    MXP = 1,
}
