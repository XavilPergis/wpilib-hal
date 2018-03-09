use std::os::raw::*;
use error::*;
use hal::types::*;
use hal::analog_trigger::AnalogTriggerType;

extern "C" {
    fn HAL_InitializeSPI(port: SpiPort, status: *mut i32);
    fn HAL_TransactionSPI(port: SpiPort, to_send: *const u8, received: *mut u8, size: i32) -> i32;
    fn HAL_WriteSPI(port: SpiPort, to_send: *const u8, size: i32) -> i32;
    fn HAL_ReadSPI(port: SpiPort, buffer: *mut u8, count: i32) -> i32;
    fn HAL_CloseSPI(port: SpiPort);
    fn HAL_SetSPISpeed(port: SpiPort, speed: i32);
    fn HAL_SetSPIOpts(port: SpiPort, msb_first: NativeBool, sample_on_trailing: NativeBool, clock_idle_high: NativeBool);
    fn HAL_SetSPIChipSelectActiveHigh(port: SpiPort, status: *mut i32);
    fn HAL_SetSPIChipSelectActiveLow(port: SpiPort, status: *mut i32);
    fn HAL_SetSPIHandle(port: SpiPort, handle: i32);
    fn HAL_GetSPIHandle(port: SpiPort) -> i32;

    fn HAL_InitSPIAuto(port: SpiPort, buffer_size: i32, status: *mut i32);
    fn HAL_FreeSPIAuto(port: SpiPort, status: *mut i32);
    fn HAL_StartSPIAutoRate(port: SpiPort, period: c_double, status: *mut i32);
    fn HAL_StartSPIAutoTrigger(port: SpiPort, digital_source: Handle, trigger_type: AnalogTriggerType, trigger_rising: NativeBool, trigger_falling: NativeBool, status: *mut i32);
    fn HAL_StopSPIAuto(port: SpiPort, status: *mut i32);
    fn HAL_SetSPIAutoTransmitData(port: SpiPort, to_send: *const u8, data_size: i32, zero_size: i32, status: *mut i32);
    fn HAL_ForceSPIAutoRead(port: SpiPort, status: *mut i32);
    fn HAL_ReadSPIAutoReceivedData(port: SpiPort, buffer: *mut u8, num_to_read: i32, timeout: c_double, status: *mut i32) -> i32;
    fn HAL_GetSPIAutoDroppedCount(port: SpiPort, status: *mut i32) -> i32;
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SpiPort {
    CS0 = 0,
    CS1,
    CS2,
    CS3,
    MXP,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SampleEdge {
    RisingEdge,
    FallingEdge,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BitDirection {
    MsbFirst,
    LsbFirst,
}

use std::io::{self, Read, Write};
use std::cell::Cell;

#[derive(Debug, Eq, PartialEq)]
pub struct Spi {
    port: SpiPort,
    msb_first: Cell<bool>,
    // When to measure the data signal based on the clock
    sample_on_trailing: Cell<bool>,
    // Whether or not a high clock signal means idle
    clock_idle_high: Cell<bool>,
}

impl Spi {
    pub fn initialize(port: SpiPort) -> HalResult<Self> {
        unsafe { hal_call!(HAL_InitializeSPI(port))?; }

        Ok(Spi {
            port,
            msb_first: Cell::new(false),
            sample_on_trailing: Cell::new(false),
            clock_idle_high: Cell::new(false)
        })
    }

    /// Do a read and write at the same time. Panics if the lengths of the buffers differ.
    pub fn transaction(&self, send_buf: &[u8], recv_buf: &mut [u8]) -> io::Result<usize> {
        if send_buf.len() != recv_buf.len() {
            panic!("Tried to do a SPI transaction but the send and received buffers differed in length. Send buffer was {} bytes long, Receive buffer was {} bytes long", send_buf.len(), recv_buf.len());
        }

        let res = unsafe {
            HAL_TransactionSPI(self.port, send_buf.as_ptr() as *const u8,
                               recv_buf.as_mut_ptr() as *mut u8, send_buf.len() as i32)
        };

        if res == -1 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(res as usize)
        }
    }

    fn update_options(&self) {
        unsafe {
            HAL_SetSPIOpts(self.port,
                           self.msb_first.get() as NativeBool,
                           self.sample_on_trailing.get() as NativeBool,
                           self.clock_idle_high.get() as NativeBool)
        }
    }

    /// Configure the SPI port to send either the most or least significant bit first.
    pub fn set_msb_first(&self, direction: BitDirection) {
        self.msb_first.set(match direction {
            BitDirection::MsbFirst => true,
            BitDirection::LsbFirst => false,
        });
        self.update_options();
    }

    /// Set on which edge of the clock signal data should be sampled.
    pub fn set_sample_edge(&self, edge: SampleEdge) {
        self.sample_on_trailing.set(match edge {
            SampleEdge::FallingEdge => true,
            SampleEdge::RisingEdge => false,
        });
        self.update_options();
    }

    /// Set whether a high clock signal means idle or active.
    pub fn set_clock_polarity(&self, polarity: Polarity) {
        self.clock_idle_high.set(match polarity {
            Polarity::ActiveHigh => false,
            Polarity::ActiveLow => true,
        });
        self.update_options();
    }

    // Set the clock rate in hertz. Panics if the speed is above the maximum value of 4,000,000.
    pub fn set_speed(&self, speed: i32) {
        require_value_between!(speed, 0, 4_000_000);
        unsafe { HAL_SetSPISpeed(self.port, speed) }
    }

    pub fn set_chip_select_polarity(&self, polarity: Polarity) -> HalResult<()> {
        match polarity {
            Polarity::ActiveHigh => unsafe { hal_call!(HAL_SetSPIChipSelectActiveHigh(self.port)) },
            Polarity::ActiveLow => unsafe { hal_call!(HAL_SetSPIChipSelectActiveLow(self.port)) },
        }
    }
}

impl Read for Spi {
    // NOTE: panics if the buffer size does not fit into an i32
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = unsafe { HAL_ReadSPI(self.port, buf.as_mut_ptr() as *mut u8, buf.len() as i32) };
        if res == -1 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(res as usize)
        }
    }
}

impl Write for Spi {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let res = unsafe { HAL_WriteSPI(self.port, buf.as_ptr() as *const u8, buf.len() as i32) };
        if res == -1 {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Ok(res as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe { HAL_CloseSPI(self.port); }
    }
}
