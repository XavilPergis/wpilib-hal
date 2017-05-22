use ::error::*;
use ::raw::*;
use std::os::raw::c_char;
use halio::RobotIoPort;

pub type RawSerialPort = HAL_SerialPort;

lazy_static! {
    static ref INITIALIZED_SERIAL_PORTS: Vec<SerialPort> = Vec::new();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SerialPort {
    OnBoard,
    MXP,
    USB1,
    USB2,
}

impl_convert! {
    HAL_SerialPort, SerialPort;
    HAL_SerialPort_Onboard <=> OnBoard,
    HAL_SerialPort_MXP <=> MXP,
    HAL_SerialPort_USB1 <=> USB1,
    HAL_SerialPort_USB2 <=> USB2
}

pub unsafe fn initialize(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_InitializeSerialPort(port.into()) ]
}

pub unsafe fn set_baud_rate(port: SerialPort, baud: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialBaudRate(port.into(), baud) ]
}

pub unsafe fn set_data_bits(port: SerialPort, bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialDataBits(port.into(), bits) ]
}

// TODO: What is parity?
pub unsafe fn set_parity(port: SerialPort, parity: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialParity(port.into(), parity) ]
}

pub unsafe fn set_stop_bits(port: SerialPort, stop_bits: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialStopBits(port.into(), stop_bits) ]
}

// TODO: What is "mode"?
pub unsafe fn set_write_mode(port: SerialPort, mode: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialWriteMode(port.into(), mode) ]
}

// TODO: What is "flow"?
pub unsafe fn set_flow_control(port: SerialPort, flow: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialFlowControl(port.into(), flow) ]
}

pub unsafe fn set_timeout(port: SerialPort, timeout: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialTimeout(port.into(), timeout) ]
}

pub unsafe fn enable_termination(port: SerialPort, terminator: u8) -> HalResult<()> {
    hal_call![ ptr HAL_EnableSerialTermination(port.into(), terminator as c_char) ]
}

pub unsafe fn disable_termination(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_DisableSerialTermination(port.into()) ]
}

pub unsafe fn set_read_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialReadBufferSize(port.into(), size) ]
}

pub unsafe fn set_write_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSerialWriteBufferSize(port.into(), size) ]
}

pub unsafe fn get_bytes_received(port: SerialPort) -> HalResult<i32> {
    hal_call![ ptr HAL_GetSerialBytesReceived(port.into()) ]
}

// The RoboRIO is ARM, so we really only need to support ARM architecture.
// c_char is u8 on ARM.
// We can't mutate a string slice because the C lib isn't required to return
// valid utf-8
// sequences, so we just store it in a u8 slice
// TODO: Maybe make this a bit more robust?

pub unsafe fn read(port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_ReadSerial(port.as_serial()?.into(), buffer.as_mut_ptr() as *mut c_char, count) ]
}

pub unsafe fn write(port: RobotIoPort, buffer: &[u8], count: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_WriteSerial(port.as_serial()?.into(), buffer.as_ptr() as *const c_char, count) ]
}

pub unsafe fn flush(port: RobotIoPort) -> HalResult<()> {
    hal_call![ ptr HAL_FlushSerial(port.as_serial()?.into()) ]
}

pub unsafe fn clear(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_ClearSerial(port.into()) ]
}

pub unsafe fn close(port: SerialPort) -> HalResult<()> {
    hal_call![ ptr HAL_CloseSerial(port.into()) ]
}
