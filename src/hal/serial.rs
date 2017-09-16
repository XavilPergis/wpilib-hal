use ::error::*;
use std::os::raw::c_char;

extern "C" {
    fn HAL_InitializeSerialPort(port: SerialPort, status: *mut i32);
    fn HAL_SetSerialBaudRate(port: SerialPort, baud: i32, status: *mut i32);
    fn HAL_SetSerialDataBits(port: SerialPort, bits: i32, status: *mut i32);
    fn HAL_SetSerialParity(port: SerialPort, parity: i32, status: *mut i32);
    fn HAL_SetSerialStopBits(port: SerialPort, stopBits: i32, status: *mut i32);
    fn HAL_SetSerialWriteMode(port: SerialPort, mode: i32, status: *mut i32);
    fn HAL_SetSerialFlowControl(port: SerialPort, flow: i32, status: *mut i32);
    fn HAL_SetSerialTimeout(port: SerialPort, timeout: ::std::os::raw::c_double, status: *mut i32);
    fn HAL_EnableSerialTermination(port: SerialPort, terminator: ::std::os::raw::c_char, status: *mut i32);
    fn HAL_DisableSerialTermination(port: SerialPort, status: *mut i32);
    fn HAL_SetSerialReadBufferSize(port: SerialPort, size: i32, status: *mut i32);
    fn HAL_SetSerialWriteBufferSize(port: SerialPort, size: i32, status: *mut i32);
    fn HAL_GetSerialBytesReceived(port: SerialPort, status: *mut i32) -> i32;
    fn HAL_ReadSerial(port: SerialPort, buffer: *mut ::std::os::raw::c_char, count: i32, status: *mut i32) -> i32;
    fn HAL_WriteSerial(port: SerialPort, buffer: *const ::std::os::raw::c_char, count: i32, status: *mut i32) -> i32;
    fn HAL_FlushSerial(port: SerialPort, status: *mut i32);
    fn HAL_ClearSerial(port: SerialPort, status: *mut i32);
    fn HAL_CloseSerial(port: SerialPort, status: *mut i32);
}

pub type RawSerialPort = SerialPort;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SerialPort {
    OnBoard = 0,
    Mxp = 1,
    Usb1 = 2,
    Usb2 = 3,
}

pub unsafe fn initialize(port: SerialPort) -> HalResult<()> {
    hal_call!(ptr HAL_InitializeSerialPort(port))
}

pub unsafe fn set_baud_rate(port: SerialPort, baud: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialBaudRate(port, baud))
}

pub unsafe fn set_data_bits(port: SerialPort, bits: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialDataBits(port, bits))
}

// TODO: What is parity?
pub unsafe fn set_parity(port: SerialPort, parity: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialParity(port, parity))
}

pub unsafe fn set_stop_bits(port: SerialPort, stop_bits: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialStopBits(port, stop_bits))
}

// TODO: What is "mode"?
pub unsafe fn set_write_mode(port: SerialPort, mode: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialWriteMode(port, mode))
}

// TODO: What is "flow"?
pub unsafe fn set_flow_control(port: SerialPort, flow: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialFlowControl(port, flow))
}

pub unsafe fn set_timeout(port: SerialPort, timeout: f64) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialTimeout(port, timeout))
}

pub unsafe fn enable_termination(port: SerialPort, terminator: u8) -> HalResult<()> {
    hal_call!(ptr HAL_EnableSerialTermination(port, terminator as c_char))
}

pub unsafe fn disable_termination(port: SerialPort) -> HalResult<()> {
    hal_call!(ptr HAL_DisableSerialTermination(port))
}

pub unsafe fn set_read_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialReadBufferSize(port, size))
}

pub unsafe fn set_write_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetSerialWriteBufferSize(port, size))
}

pub unsafe fn get_bytes_received(port: SerialPort) -> HalResult<i32> {
    hal_call!(ptr HAL_GetSerialBytesReceived(port))
}

pub unsafe fn read(port: SerialPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    hal_call!(ptr HAL_ReadSerial(port, buffer.as_mut_ptr() as *mut c_char, count))
}

pub unsafe fn write(port: SerialPort, buffer: &[u8], count: i32) -> HalResult<i32> {
    hal_call!(ptr HAL_WriteSerial(port, buffer.as_ptr() as *const c_char, count))
}

pub unsafe fn flush(port: SerialPort) -> HalResult<()> {
    hal_call!(ptr HAL_FlushSerial(port))
}

pub unsafe fn clear(port: SerialPort) -> HalResult<()> {
    hal_call!(ptr HAL_ClearSerial(port))
}

pub unsafe fn close(port: SerialPort) -> HalResult<()> {
    hal_call!(ptr HAL_CloseSerial(port))
}
