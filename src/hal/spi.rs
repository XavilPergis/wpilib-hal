use ::error::*;
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

// TODO: handle thingy?
pub fn initialize_spi(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeSPI(port) ] }
}

pub fn write_spi(port: i32, data_to_send: &[u8], send_size: i32) -> i32 {
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
    unsafe { hal_call![ ptr HAL_SetSPIChipSelectActiveHigh(port) ] }
}

pub fn set_spi_chip_select_active_low(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIChipSelectActiveLow(port) ] }
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
    unsafe { hal_call![ ptr HAL_InitSPIAccumulator(port, opts.period, opts.cmd, opts.transfer_size, opts.valid_mask, opts.valid_value, opts.data_shift, opts.data_size, opts.is_signed as HAL_Bool, opts.big_endian as HAL_Bool) ] }
}

fn free_spi_accumulator(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_FreeSPIAccumulator(port) ] }
}

fn reset_spi_accumulator(port: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_ResetSPIAccumulator(port) ] }
}

fn set_spi_accumulator_center(port: i32, center: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIAccumulatorCenter(port, center) ] }
}

fn set_spi_accumulator_deadband(port: i32, deadband: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIAccumulatorDeadband(port, deadband) ] }
}

fn hal_get_spi_accumulator_last_value(port: i32) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorLastValue(port) ] }
}

fn hal_get_spi_accumulator_value(port: i32) -> HalResult<i64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorValue(port) ] }
}

fn hal_get_spi_accumulator_count(port: i32) -> HalResult<i64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorCount(port) ] }
}

fn hal_get_spi_accumulator_average(port: i32) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorAverage(port) ] }
}

fn get_spi_accumulator_output(port: i32, mut value: i64, mut count: i64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorOutput(port, &mut value, &mut count) ] }
}
