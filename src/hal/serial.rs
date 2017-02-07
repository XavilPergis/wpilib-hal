use ::raw::*;

pub fn initialize_serial_port(port: SerialPort) {
    unsafe { HAL_InitializeSerialPort(port.into()) }
}

pub fn set_serial_baud_rate(port: SerialPort, baud: i32) {
    unsafe { HAL_SetSerialBaudRate(port.into(), baud) }
}

pub fn set_serial_data_bits(port: SerialPort, bits: i32) {
    unsafe { HAL_SetSerialDataBits(port.into(), bits) }
}

// TODO: What is parity?
pub fn set_serial_parity(port: SerialPort, parity: i32) {
    unsafe { HAL_SetSerialParity(port.into(), parity) }
}

pub fn set_serial_stop_bits(port: SerialPort, stop_bits: i32) {
    unsafe { HAL_SetSerialStopBits(port.into(), stop_bits) }
}

// TODO: What is "mode"?
pub fn set_serial_write_mode(port: SerialPort, mode: i32) {
    unsafe { HAL_SetSerialWriteMode(port.into(), mode) }
}

// TODO: What is "flow"?
pub fn set_serial_flow_control(port: SerialPort, flow: i32) {
    unsafe { HAL_SetSerialFlowControl(port.into(), flow) }
}

pub fn set_serial_timeout(port: SerialPort, timeout: f64) {
    unsafe { HAL_SetSerialTimeout(port.into(), timeout) }
}

pub fn enable_serial_termination(port: SerialPort, terminator: u8) {
    unsafe { HAL_EnableSerialTermination(port.into(), terminator) }
}

pub fn disable_serial_termination(port: SerialPort) {
    unsafe { HAL_DisableSerialTermination(port.into()) }
}

pub fn set_serial_read_buffer_size(port: SerialPort, size: i32) {
    unsafe { HAL_SetSerialReadBufferSize(port.into(), size) }
}

pub fn set_serial_write_buffer_size(port: SerialPort, size: i32) {
    unsafe { HAL_SetSerialWriteBufferSize(port.into(), size) }
}

pub fn get_serial_bytes_received(port: SerialPort) -> HalResult<i32> {
    hal_call![ ptr HAL_GetSerialBytesReceived(port.into()) ]
}

pub fn read_serial(port: SerialPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    // The RoboRIO is ARM, so we really only need to support ARM architecture. c_char is u8 on ARM.
    // We can't mutate a string slice because the C lib isn't required to return valid utf-8
    // sequences, so we just store it in a u8 slice
    // TODO: Maybe make this a bit more robust?
    hal_call![ ptr HAL_ReadSerial(port.into(), buffer.as_mut_ptr(), count) ]
}

pub fn write_serial(port: SerialPort, buffer: &[u8], count: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_WriteSerial(port.into(), buffer.as_ptr(), count) ]
}

pub fn flush_serial(port: SerialPort) {
    unsafe { HAL_FlushSerial(port.into()) }
}

pub fn clear_serial(port: SerialPort) {
    unsafe { HAL_ClearSerial(port.into()) }
}

pub fn close_serial(port: SerialPort) {
    unsafe { HAL_CloseSerial(port.into()) }
}
