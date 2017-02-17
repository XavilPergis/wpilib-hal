use ::error::*;
use ::raw::*;
use std::os::raw::c_char;
use serial_io::RobotIoPort;

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

impl SerialPort {
    pub fn into_raw(&self) -> RawSerialPort {
        match *self {
            SerialPort::OnBoard => HAL_SerialPort::HAL_SerialPort_Onboard,
            SerialPort::MXP => HAL_SerialPort::HAL_SerialPort_MXP,
            SerialPort::USB1 => HAL_SerialPort::HAL_SerialPort_USB1,
            SerialPort::USB2 => HAL_SerialPort::HAL_SerialPort_USB2,
        }
    }
}

impl From<RawSerialPort> for SerialPort {
    fn from(raw: RawSerialPort) -> Self {
        match raw {
            HAL_SerialPort::HAL_SerialPort_Onboard => SerialPort::OnBoard,
            HAL_SerialPort::HAL_SerialPort_MXP => SerialPort::MXP,
            HAL_SerialPort::HAL_SerialPort_USB1 => SerialPort::USB1,
            HAL_SerialPort::HAL_SerialPort_USB2 => SerialPort::USB2,
        }
    }
}

fn initialize_serial_port(port: SerialPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeSerialPort(port.into_raw()) ] }
}

fn set_serial_baud_rate(port: SerialPort, baud: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialBaudRate(port.into_raw(), baud) ] }
}

fn set_serial_data_bits(port: SerialPort, bits: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialDataBits(port.into_raw(), bits) ] }
}

// TODO: What is parity?
fn set_serial_parity(port: SerialPort, parity: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialParity(port.into_raw(), parity) ] }
}

fn set_serial_stop_bits(port: SerialPort, stop_bits: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialStopBits(port.into_raw(), stop_bits) ] }
}

// TODO: What is "mode"?
fn set_serial_write_mode(port: SerialPort, mode: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialWriteMode(port.into_raw(), mode) ] }
}

// TODO: What is "flow"?
fn set_serial_flow_control(port: SerialPort, flow: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialFlowControl(port.into_raw(), flow) ] }
}

fn set_serial_timeout(port: SerialPort, timeout: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialTimeout(port.into_raw(), timeout) ] }
}

fn enable_serial_termination(port: SerialPort, terminator: u8) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_EnableSerialTermination(port.into_raw(), terminator as c_char) ] }
}

fn disable_serial_termination(port: SerialPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_DisableSerialTermination(port.into_raw()) ] }
}

fn set_serial_read_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialReadBufferSize(port.into_raw(), size) ] }
}

fn set_serial_write_buffer_size(port: SerialPort, size: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSerialWriteBufferSize(port.into_raw(), size) ] }
}

fn get_serial_bytes_received(port: SerialPort) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetSerialBytesReceived(port.into_raw()) ] }
}

// The RoboRIO is ARM, so we really only need to support ARM architecture.
// c_char is u8 on ARM.
// We can't mutate a string slice because the C lib isn't required to return
// valid utf-8
// sequences, so we just store it in a u8 slice
// TODO: Maybe make this a bit more robust?

pub fn read_serial(port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    if let RobotIoPort::Serial(p) = port {
        unsafe { hal_call![ ptr HAL_ReadSerial(p.into_raw(), buffer.as_mut_ptr() as *mut c_char, count) ] }
    } else {
        Err(HalError::WrongIoInterface)
    }
}

pub fn write_serial(port: RobotIoPort, buffer: &[u8], count: i32) -> HalResult<i32> {
    if let RobotIoPort::Serial(p) = port {
        unsafe { hal_call![ ptr HAL_WriteSerial(p.into_raw(), buffer.as_ptr() as *const c_char, count) ] }
    } else {
        Err(HalError::WrongIoInterface)
    }
}

pub fn flush_serial(port: RobotIoPort) -> HalResult<()> {
    if let RobotIoPort::Serial(p) = port {
        unsafe { hal_call![ ptr HAL_FlushSerial(p.into_raw()) ] }
    } else {
        Err(HalError::WrongIoInterface)
    }
}

fn clear_serial(port: SerialPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_ClearSerial(port.into_raw()) ] }
}

fn close_serial(port: SerialPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_CloseSerial(port.into_raw()) ] }
}
