use ::error::*;
use hal::serial_io::HalSerialIO;
use ::raw::*;

lazy_static! {
    static ref INITIALIZED_SPI_PORTS: Vec<i32> = Vec::new();
}

/// Which port the SPI is on
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SpiPort {
    CS0,
    CS1,
    CS2,
    CS3,
    MXP,
    Unknown(i32),
}

impl SpiPort {
    pub fn get_port(&self) -> i32 {
        match *self {
            SpiPort::CS0 => 0,
            SpiPort::CS1 => 1,
            SpiPort::CS2 => 2,
            SpiPort::CS3 => 3,
            SpiPort::MXP => 4,
            SpiPort::Unknown(k) => k,
        }
    }
}

impl From<i32> for SpiPort {
    fn from(i: i32) -> SpiPort {
        match i {
            0 => SpiPort::CS0,
            1 => SpiPort::CS1,
            2 => SpiPort::CS2,
            3 => SpiPort::CS3,
            4 => SpiPort::MXP,
            k => SpiPort::Unknown(k),
        }
    }
}

/// Options for an SPI
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SpiOptions {
    /// The number of bytes to read per call
    pub read_size: i32,
}

impl Default for SpiOptions {
    fn default() -> Self {
        SpiOptions { read_size: 1 }
    }
}

/// Represents an SPI on the robot; there should only ever be the 4 on the
/// RoboRIO
pub struct HalSpi {
    /// The port of this SPI
    port: i32,
    /// Options for this SPI
    opts: SpiOptions,
}

impl HalSpi {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: SpiPort) -> HalResult<HalSpi> {
        if INITIALIZED_SPI_PORTS.contains(&port.get_port()) {
            Err(HalError::ResourceAlreadyInitialized)
        } else {
            self::initialize_spi(port.get_port())?;

            Ok(HalSpi {
                port: port.get_port(),
                opts: Default::default(),
            })
        }
    }

    /// Creates a new SPI instance from a port number
    ///
    /// # Safety
    /// Trying to read or write to the same SPI port at the same time from two
    /// different threads
    /// could lead to data races. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, opts: SpiOptions) -> HalResult<HalSpi> {
        self::initialize_spi(port)?;

        Ok(HalSpi {
            port: port,
            opts: opts,
        })
    }

    /// Set the clock speed of this SPI
    pub fn set_speed(&self, speed: i32) {
        self::set_spi_speed(self.get_handle(), speed)
    }

    /// Set this SPI's options
    pub fn set_opts(&self, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
        self::set_spi_opts(self.port, msb_first, sample_on_trailing, clock_idle_high);
    }

    pub fn set_chip_select_active_high(&self) -> HalResult<()> {
        self::set_spi_chip_select_active_high(self.port)
    }

    /// Get this SPI's handle
    pub fn get_handle(&self) -> i32 {
        self::get_spi_handle(self.port)
    }

    /// Set this SPI's handle
    pub fn set_handle(&self, handle: i32) {
        self::set_spi_handle(self.port, handle);
    }
}

impl HalSerialIO for HalSpi {
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32> {
        Ok(self::read_spi(self.port, buf, self.opts.read_size))
    }

    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32> {
        Ok(self::write_spi(self.port, buf, buf.len() as i32))
    }
}

impl Drop for HalSpi {
    fn drop(&mut self) {
        self::close_spi(self.port)
    }
}


// TODO: handle thingy?
pub fn initialize_spi(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_InitializeSPI(port) ]
}

pub fn write_spi(port: i32, data_to_send: &[u8], send_size: i32) -> i32 {
    // trick the compiler; the data we're sending *isn't actually mutable*
    unsafe { HAL_WriteSPI(port, data_to_send.as_ptr() as *mut u8, send_size) }
}

pub fn read_spi(port: i32, buffer: &mut [u8], count: i32) -> i32 {
    unsafe { HAL_ReadSPI(port, buffer.as_mut_ptr(), count) }
}

pub fn close_spi(port: i32) {
    unsafe { HAL_CloseSPI(port) }
}

pub fn set_spi_speed(port: i32, speed: i32) {
    unsafe { HAL_SetSPISpeed(port, speed) }
}

pub fn set_spi_opts(port: i32, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
    unsafe {
        HAL_SetSPIOpts(port,
                       msb_first as HAL_Bool,
                       sample_on_trailing as HAL_Bool,
                       clock_idle_high as HAL_Bool)
    }
}

pub fn set_spi_chip_select_active_high(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSPIChipSelectActiveHigh(port) ]
}

pub fn set_spi_chip_select_active_low(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSPIChipSelectActiveLow(port) ]
}

pub fn get_spi_handle(port: i32) -> i32 {
    unsafe { HAL_GetSPIHandle(port) }
}

pub fn set_spi_handle(port: i32, handle: i32) {
    unsafe { HAL_SetSPIHandle(port, handle) }
}

#[derive(Debug)]
pub struct SpiAccumulatorOptions {
    period: i32,
    cmd: i32,
    transfer_size: i32,
    valid_mask: i32,
    valid_value: i32,
    data_shift: i32,
    data_size: i32,
    is_signed: bool,
    big_endian: bool,
}

fn init_spi_accumulator(port: i32, opts: SpiAccumulatorOptions) -> HalResult<()> {
    hal_call![ ptr HAL_InitSPIAccumulator(port, opts.period, opts.cmd, opts.transfer_size, opts.valid_mask, opts.valid_value, opts.data_shift, opts.data_size, opts.is_signed as HAL_Bool, opts.big_endian as HAL_Bool) ]
}

fn free_spi_accumulator(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_FreeSPIAccumulator(port) ]
}

fn reset_spi_accumulator(port: i32) -> HalResult<()> {
    hal_call![ ptr HAL_ResetSPIAccumulator(port) ]
}

fn set_spi_accumulator_center(port: i32, center: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSPIAccumulatorCenter(port, center) ]
}

fn set_spi_accumulator_deadband(port: i32, deadband: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetSPIAccumulatorDeadband(port, deadband) ]
}

fn hal_get_spi_accumulator_last_value(port: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_GetSPIAccumulatorLastValue(port) ]
}

fn hal_get_spi_accumulator_value(port: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetSPIAccumulatorValue(port) ]
}

fn hal_get_spi_accumulator_count(port: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetSPIAccumulatorCount(port) ]
}

fn hal_get_spi_accumulator_average(port: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetSPIAccumulatorAverage(port) ]
}

fn get_spi_accumulator_output(port: i32, mut value: i64, mut count: i64) -> HalResult<()> {
    hal_call![ ptr HAL_GetSPIAccumulatorOutput(port, &mut value, &mut count) ]
}
