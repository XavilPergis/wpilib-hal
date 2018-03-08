use error::*;
use std::os::raw::*;

extern "C" {
    fn HAL_InitializeSerialPort(port: Port, status: *mut i32);
    fn HAL_SetSerialBaudRate(port: Port, baud: i32, status: *mut i32);
    fn HAL_SetSerialDataBits(port: Port, bits: i32, status: *mut i32);
    fn HAL_SetSerialParity(port: Port, parity: i32, status: *mut i32);
    fn HAL_SetSerialStopBits(port: Port, stopBits: i32, status: *mut i32);
    fn HAL_SetSerialWriteMode(port: Port, mode: i32, status: *mut i32);
    fn HAL_SetSerialFlowControl(port: Port, flow: i32, status: *mut i32);
    fn HAL_SetSerialTimeout(port: Port, timeout: c_double, status: *mut i32);
    fn HAL_EnableSerialTermination(port: Port, terminator: c_char, status: *mut i32);
    fn HAL_DisableSerialTermination(port: Port, status: *mut i32);
    fn HAL_SetSerialReadBufferSize(port: Port, size: i32, status: *mut i32);
    fn HAL_SetSerialWriteBufferSize(port: Port, size: i32, status: *mut i32);
    fn HAL_GetSerialBytesReceived(port: Port, status: *mut i32) -> i32;
    fn HAL_ReadSerial(port: Port, buffer: *mut c_char, count: i32, status: *mut i32) -> i32;
    fn HAL_WriteSerial(port: Port, buffer: *const c_char, count: i32, status: *mut i32) -> i32;
    fn HAL_FlushSerial(port: Port, status: *mut i32);
    fn HAL_ClearSerial(port: Port, status: *mut i32);
    fn HAL_CloseSerial(port: Port, status: *mut i32);
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum Port {
    Onboard = 0,
    MXP = 1,
    USB1 = 2,
    USB2 = 3,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StopBits {
    One = 10,
    OnePointFive = 15,
    Two = 20,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum FlowControl {
    None = 0,
    XonXoff = 1,
    RtsCts = 2,
    DtsDsr = 4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WriteMode {
    FlushOnAccess = 1,
    FlushWhenFull = 2,
}

#[derive(Debug)]
pub struct Serial {
    pub(crate) port: Port,
}

impl Serial {
    pub fn initialize(port: Port) -> HalResult<Self> {
        unsafe { hal_call!(HAL_InitializeSerialPort(port))?; }

        Ok(Serial { port })
    }

    pub fn set_baud_rate(&self, rate: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialBaudRate(self.port, rate)) }
    }

    /// Set the amount of data bits per transfer. Valid values are between 5 and 8.
    pub fn set_data_bits(&self, bits: i32) -> HalResult<()> {
        if bits > 8 || bits < 5 {
            panic!("Serial data bits out of range. Expected range is [5, 8], but actual value was {}", bits);
        }

        unsafe { hal_call!(HAL_SetSerialDataBits(self.port, bits)) }
    }

    pub fn set_parity(&self, parity: Parity) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialParity(self.port, parity as i32)) }
    }

    pub fn set_stop_bits(&self, bits: StopBits) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialStopBits(self.port, bits as i32)) }
    }

    pub fn set_write_mode(&self, mode: WriteMode) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialWriteMode(self.port, mode as i32)) }
    }

    pub fn set_flow_control(&self, flow: FlowControl) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialFlowControl(self.port, flow as i32)) }
    }

    pub fn set_timeout(&self, timeout: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialTimeout(self.port, timeout)) }
    }

    pub fn enable_termination(&self, terminator: u8) -> HalResult<()> {
        unsafe { hal_call!(HAL_EnableSerialTermination(self.port, terminator as c_char)) }
    }

    pub fn disable_termination(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_DisableSerialTermination(self.port)) }
    }

    pub fn set_read_buffer_size(&self, size: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialReadBufferSize(self.port, size)) }
    }

    pub fn set_write_buffer_size(&self, size: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSerialWriteBufferSize(self.port, size)) }
    }

    pub fn get_serial_bytes_received(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetSerialBytesReceived(self.port)) }
    }

    // pub fn HAL_ReadSerial(port: Port, buffer: *mut c_char, count: i32, status: *mut i32) -> i32;
    pub fn read(&self, buffer: &mut [u8]) -> HalResult<i32> {
        unsafe { hal_call!(HAL_ReadSerial(self.port, buffer.as_mut_ptr() as *mut c_char, buffer.len() as i32)) }
    }

    // pub fn HAL_WriteSerial(port: Port, buffer: *const c_char, count: i32, status: *mut i32) -> i32;
    pub fn write(&self, buffer: &[u8]) -> HalResult<i32> {
        unsafe { hal_call!(HAL_WriteSerial(self.port, buffer.as_ptr() as *const c_char, buffer.len() as i32)) }
    }

    pub fn flush(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_FlushSerial(self.port)) }
    }

    pub fn clear(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_ClearSerial(self.port)) }
    }
}

impl Drop for Serial {
    fn drop(&mut self) {
        unsafe { hal_call!(HAL_CloseSerial(self.port)).unwrap() }
    }
}
