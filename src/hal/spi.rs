use ::error::*;
use ::raw::*;
use halio::RobotIoPort;

/// Which port the SPI is on
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

pub fn initialize(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitializeSPI(port.get_port()) ] }
}

pub fn transaction(port: i32, send_buffer: &[u8], recv_buffer: &mut [u8], size: i32) -> i32 {
    unsafe { HAL_TransactionSPI(port, send_buffer.as_ptr() as *mut u8, recv_buffer.as_mut_ptr(), size) }
}

pub fn write(port: RobotIoPort, buffer: &[u8], send_size: i32) -> HalResult<i32> {
    unsafe { Ok(HAL_WriteSPI(port.as_spi()?.get_port(), buffer.as_ptr() as *mut u8, send_size)) }
}

pub fn read(port: RobotIoPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
    unsafe { Ok(HAL_ReadSPI(port.as_spi()?.get_port(), buffer.as_mut_ptr(), count)) }
}

pub fn close(port: RobotIoPort) -> HalResult<()> {
    unsafe { Ok(HAL_CloseSPI(port.as_spi()?.get_port())) }
}

pub fn set_speed(port: SpiPort, speed: i32) {
    unsafe { HAL_SetSPISpeed(port.get_port(), speed) }
}

pub fn set_opts(port: SpiPort, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
    unsafe {
        HAL_SetSPIOpts(port.get_port(),
                       msb_first as HAL_Bool,
                       sample_on_trailing as HAL_Bool,
                       clock_idle_high as HAL_Bool)
    }
}

pub fn set_chip_select_active_high(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIChipSelectActiveHigh(port.get_port()) ] }
}

pub fn set_chip_select_active_low(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIChipSelectActiveLow(port.get_port()) ] }
}

pub fn get_handle(port: SpiPort) -> i32 {
    unsafe { HAL_GetSPIHandle(port.get_port()) }
}

pub fn set_handle(port: SpiPort, handle: i32) {
    unsafe { HAL_SetSPIHandle(port.get_port(), handle) }
}

/// Options for an SPI Accumulator
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

pub fn init_accumulator(port: SpiPort, opts: SpiAccumulatorOptions) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_InitSPIAccumulator(port.get_port(), opts.period, opts.cmd, opts.transfer_size, opts.valid_mask, opts.valid_value, opts.data_shift, opts.data_size, opts.is_signed as HAL_Bool, opts.big_endian as HAL_Bool) ] }
}

pub fn free_accumulator(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_FreeSPIAccumulator(port.get_port()) ] }
}

pub fn reset_accumulator(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_ResetSPIAccumulator(port.get_port()) ] }
}

pub fn set_accumulator_center(port: SpiPort, center: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIAccumulatorCenter(port.get_port(), center) ] }
}

pub fn set_accumulator_deadband(port: SpiPort, deadband: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetSPIAccumulatorDeadband(port.get_port(), deadband) ] }
}

pub fn hal_get_accumulator_last_value(port: SpiPort) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorLastValue(port.get_port()) ] }
}

pub fn hal_get_accumulator_value(port: SpiPort) -> HalResult<i64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorValue(port.get_port()) ] }
}

pub fn hal_get_accumulator_count(port: SpiPort) -> HalResult<i64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorCount(port.get_port()) ] }
}

pub fn hal_get_accumulator_average(port: SpiPort) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorAverage(port.get_port()) ] }
}

pub fn get_accumulator_output(port: SpiPort, mut value: i64, mut count: i64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_GetSPIAccumulatorOutput(port.get_port(), &mut value, &mut count) ] }
}
