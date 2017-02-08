use ::raw::*;
use hal::serial_io::SerialPort;
use ::error::*;
use std::os::raw::c_char;

pub fn initialize_serial_port(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_InitializeSerialPort(port.into_raw()) ]
}

pub fn set_serial_baud_rate(port: SerialPort, baud: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialBaudRate(port.into_raw(), baud) ]
}

pub fn set_serial_data_bits(port: SerialPort, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialDataBits(port.into_raw(), bits) ]
}

// TODO: What is parity?
pub fn set_serial_parity(port: SerialPort, parity: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialParity(port.into_raw(), parity) ]
}

pub fn set_serial_stop_bits(port: SerialPort, stop_bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialStopBits(port.into_raw(), stop_bits) ]
}

// TODO: What is "mode"?
pub fn set_serial_write_mode(port: SerialPort, mode: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialWriteMode(port.into_raw(), mode) ]
}

// TODO: What is "flow"?
pub fn set_serial_flow_control(port: SerialPort, flow: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialFlowControl(port.into_raw(), flow) ]
}

pub fn set_serial_timeout(port: SerialPort, timeout: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialTimeout(port.into_raw(), timeout) ]
}

pub fn enable_serial_termination(port: SerialPort, terminator: u8) -> HalResult<()> {
    hal_call![ ptr HAL_EnableSerialTermination(port.into_raw(), terminator as c_char) ]
}

pub fn disable_serial_termination(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_DisableSerialTermination(port.into_raw()) ]
}

pub fn set_serial_read_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialReadBufferSize(port.into_raw(), size) ]
}

pub fn set_serial_write_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialWriteBufferSize(port.into_raw(), size) ]
}

pub fn get_serial_bytes_received(port: SerialPort) -> HalResult<i32> {
    hal_call![ ptr HAL_GetSerialBytesReceived(port.into_raw()) ]
}


pub fn read_serial(port: SerialPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    // The RoboRIO is ARM, so we really only need to support ARM architecture. c_char is u8 on ARM.
    // We can't mutate a string slice because the C lib isn't required to return valid utf-8
    // sequences, so we just store it in a u8 slice
    // TODO: Maybe make this a bit more robust?
    hal_call![ ptr HAL_ReadSerial(port.into_raw(), buffer.as_mut_ptr() as *mut c_char, count) ]
}

pub fn write_serial(port: SerialPort, buffer: &[u8], count: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_WriteSerial(port.into_raw(), buffer.as_ptr() as *const c_char, count) ]
}

pub fn flush_serial(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_FlushSerial(port.into_raw()) ]
}

pub fn clear_serial(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_ClearSerial(port.into_raw()) ]
}

pub fn close_serial(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_CloseSerial(port.into_raw()) ]
}
