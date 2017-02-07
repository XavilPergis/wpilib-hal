use ::raw::*;

pub fn initialize_i2c(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_InitializeI2C(port) ]
}

// TODO
//pub fn transaction_i2c(port: i32, device_address: i32, dataToSend: *mut u8, sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32 {
//
//}

pub fn write_i2c(port: i32, device_address: i32, data_to_send: &[u8], send_size: i32) -> i32 {
    unsafe { HAL_WriteI2C(port, device_address, data_to_send.as_ptr() as *mut u8, send_size) }
}

pub fn read_i2c(port: i32, device_address: i32, buffer: &mut [u8], count: i32) -> i32 {
    unsafe { HAL_ReadI2C(port, device_address, buffer.as_mut_ptr() as *mut u8, count) }
}

pub fn close_i2c(port: i32) {
    unsafe { HAL_CloseI2C(port) }
}