use std::io::{ self, Read, Write, ErrorKind };
use std::iter::Iterator;
use std::io::Error as IoError;
use std::error::Error;
use hal::wrapper::i2c;

lazy_static! {
    static ref INITIALIZED_I2C_PORTS: Vec<i32> = Vec::new();
}

/// Which port the SPI is plugged into
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum I2cPort {
    OnBoard,
    MXP
}

impl SpiPort {
    pub fn get_port(&self) -> i32 {
        match self {
            I2cPort::OnBoard => 0,
            I2cPort::MXP => 1
        }
    }
}

/// Options for an I2C
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct I2cOptions {
    /// The number of bytes to read per call
    pub read_size: i32
}

impl Default for I2cOptions {
    fn default() -> Self {
        I2cOptions {
            read_size: 1
        }
    }
}

/// Shallow I2C IO error
#[derive(Copy, Clone, Debug)]
pub enum I2cError {
    /// Error reading from an I2C
    ReadError,
    /// Error writing to an I2C
    WriteError
}

impl Error for I2cError {
    fn description(&self) -> &str {
        match self {
            I2cError::ReadError => "I2C Read error",
            I2cError::WriteError => "I2C Write Error"
        }
    }
}

/// Represents an SPI on the robot; there should only ever be the 4 on the RoboRIO
pub struct I2C {
    /// The port of this SPI
    port: i32,
    /// Options for this SPI
    opts: I2cOptions
}

impl I2C {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: I2cPort) -> Option<I2C> {
        if INITIALIZED_I2C_PORTS.contains(port.get_port()) {
            None
        } else {
            i2c::initialize_i2c(port.get_port());

            Some(I2C {
                port: port.get_port(),
                opts: Default::default()
            })
        }
    }

    /// Creates a new SPI instance from a port number
    ///
    /// ## Unsafety
    /// Trying to read or write to the same SPI port at the same time from two different threads
    /// would cause a data race. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, opts: I2cOptions) -> I2C {
        i2c::initialize_i2c(port);

        I2C {
            port: port,
            opts: opts
        }
    }
}

impl Read for I2C {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = i2c::read_i2c(self.port, buf, self.opts.size);

        if read <= -1 {
            Err(IoError::new(ErrorKind::Other, I2cError::ReadError))
        } else {
            Ok(read as usize)
        }
    }
}

impl Write for I2C {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = i2c::write_i2c(self.port, buf, buf.len());

        if written <= -1 {
            Err(IoError::new(ErrorKind::Other, I2cError::WriteError))
        } else {
            Ok(written as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for I2C {
    fn drop(&mut self) {
        i2c::close_i2c(self.port);
    }
}
