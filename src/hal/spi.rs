use std::io::{ self, Read, Write, ErrorKind };
use std::iter::Iterator;
use std::io::Error as IoError;
use std::error::Error;
use raw::{ HAL_ReadSPI, HAL_WriteSPI, HAL_CloseSPI, HAL_SetSPISpeed };
use hal::wrapper::spi;

lazy_static! {
    static ref INITIALIZED_SPI_PORTS: Vec<i32> = Vec::new();
}

/// Which port the SPI is plugged into
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SpiPort {
    CS0,
    CS1,
    CS2,
    CS3,
    MXP
}

impl SpiPort {
    pub fn get_port(&self) -> i32 {
        match self {
            SpiPort::CS0 => 0,
            SpiPort::CS1 => 1,
            SpiPort::CS2 => 2,
            SpiPort::CS3 => 3,
            SpiPort::MXP => 4
        }
    }
}

/// Options for an SPI
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SpiOptions {
    /// The number of bytes to read per call
    pub read_size: i32
}

impl Default for SpiOptions {
    fn default() -> Self {
        SpiOptions {
            read_size: 1
        }
    }
}

/// Shallow SPI IO error
#[derive(Copy, Clone, Debug)]
pub enum SpiError {
    /// Error reading from an SPI
    ReadError,
    /// Error writing to an SPI
    WriteError
}

impl Error for SpiError {
    fn description(&self) -> &str {
        match self {
            SpiError::ReadError => "SPI Read error",
            SpiError::WriteError => "SPI Write Error"
        }
    }
}

/// Represents an SPI on the robot; there should only ever be the 4 on the RoboRIO
pub struct HalSpi {
    /// The port of this SPI
    port: SpiPort,
    /// Options for this SPI
    opts: SpiOptions
}

impl HalSpi {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: SpiPort) -> Option<HalSpi> {
        if INITIALIZED_SPI_PORTS.contains(port.get_port()) {
            None
        } else {
            spi::initialize_spi(port.get_port());

            Some(HalSpi {
                port: port,
                opts: Default::default()
            })
        }
    }

    /// Creates a new SPI instance from a port number
    ///
    /// ## Unsafety
    /// Trying to read or write to the same SPI port at the same time from two different threads
    /// would cause a data race. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, opts: SpiOptions) -> HalSpi {
        spi::initialize_spi(port);

        HalSpi {
            port: port,
            opts: opts
        }
    }

    /// Set the clock speed of this SPI
    pub fn set_speed(&self, speed: i32) {
        unsafe { HAL_SetSPISpeed(self.port, speed) };
    }

    /// Set this SPI's options
    pub fn set_opts(&self, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
        spi::set_spi_opts(self.port, msb_first, sample_on_trailing, clock_idle_high);
    }

    pub fn set_chip_select_active_high(&self) {
        spi::set_spi_chip_select_active_high(self.port);
    }

    /// Get this SPI's handle
    pub fn get_handle(&self) -> i32 {
        spi::get_spi_handle(self.port)
    }

    /// Set this SPI's handle
    pub fn set_handle(&self, handle: i32) {
        spi::set_spi_handle(self.port, handle);
    }
}

// TODO: Cross-thread sharing?
// Probably can't share across threads
impl !Sync for HalSpi {}
impl !Sync for HalSpi {}

impl Read for HalSpi {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = spi::write_spi(self.port, buf, self.opts.size);

        if read <= -1 {
            Err(IoError::new(ErrorKind::Other, SpiError::ReadError))
        } else {
            Ok(read as usize)
        }
    }
}

impl Write for HalSpi {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = spi::write_spi(self.port, buf, buf.len());

        if written <= -1 {
            Err(IoError::new(ErrorKind::Other, SpiError::WriteError))
        } else {
            Ok(written as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for HalSpi {
    fn drop(&mut self) {
        HAL_CloseSPI(self.port);
    }
}
