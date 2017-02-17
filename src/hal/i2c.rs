use ::error::*;
use hal::serial_io::HalSerialIO;
use ::raw::*;

lazy_static! {
    static ref INITIALIZED_I2C_PORTS: Vec<i32> = Vec::new();
}

/// Which port the I2C device is on
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum I2cPort {
    OnBoard,
    MXP,
}

impl I2cPort {
    pub fn get_port(&self) -> i32 {
        match *self {
            I2cPort::OnBoard => 0,
            I2cPort::MXP => 1,
        }
    }
}

/// Options for an I2C
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct I2cOptions {
    /// The number of bytes to read per call
    pub read_size: i32,
}

impl Default for I2cOptions {
    fn default() -> Self {
        I2cOptions { read_size: 1 }
    }
}

/// Represents an I2C on the robot; there should only ever be the 4 on the
/// RoboRIO
pub struct HalI2C {
    /// The port of this I2C
    port: i32,
    /// TODO: What is this?
    address: i32,
    /// Options for this I2C
    opts: I2cOptions,
}

impl HalI2C {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: I2cPort, address: i32) -> HalResult<HalI2C> {
        HalI2C::new_with_opts(port, address, Default::default())
    }

    /// Construct and initialize a serial port using passed in options
    pub fn new_with_opts(port: I2cPort, address: i32, opts: I2cOptions) -> HalResult<HalI2C> {
        if INITIALIZED_I2C_PORTS.contains(&port.get_port()) {
            Err(HalError::ResourceAlreadyInitialized)
        } else {
            self::initialize_i2c(port.get_port())?;

            Ok(HalI2C {
                port: port.get_port(),
                address: address,
                opts: opts,
            })
        }
    }

    /// Creates a new I2C instance from a port number
    ///
    /// # Safety
    /// Trying to read or write to the same I2C port at the same time from two
    /// different threads
    /// would cause a data race. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, address: i32, opts: I2cOptions) -> HalResult<HalI2C> {
        self::initialize_i2c(port)?;

        Ok(HalI2C {
            port: port,
            address: address,
            opts: opts,
        })
    }
}

impl HalSerialIO for HalI2C {
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32> {
        Ok(self::read_i2c(self.port, self.address, buf, self.opts.read_size))
    }

    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32> {
        Ok(self::write_i2c(self.port, self.address, buf, buf.len() as i32))
    }
}

impl Drop for HalI2C {
    fn drop(&mut self) {
        self::close_i2c(self.port)
    }
}

fn initialize_i2c(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_InitializeI2C(port) ]
}

// TODO
// fn transaction_i2c(port: i32, device_address: i32, dataToSend: *mut u8,
// sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32 {
//
//

fn write_i2c(port: i32, device_address: i32, data_to_send: &[u8], send_size: i32) -> i32 {
    unsafe {
        HAL_WriteI2C(port, device_address, data_to_send.as_ptr() as *mut u8, send_size)
    }
}

fn read_i2c(port: i32, device_address: i32, buffer: &mut [u8], count: i32) -> i32 {
    unsafe { HAL_ReadI2C(port, device_address, buffer.as_mut_ptr(), count) }
}

fn close_i2c(port: i32) {
    unsafe { HAL_CloseI2C(port) }
}
