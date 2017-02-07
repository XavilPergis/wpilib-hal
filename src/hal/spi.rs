use ::raw::*;

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
    unsafe { raw::HAL_CloseSPI(port) }
}

pub fn set_spi_speed(port: i32, speed: i32) {
    unsafe { HAL_SetSPISpeed(port, speed) }
}

pub fn set_spi_opts(port: i32, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
    unsafe { HAL_SetSPIOpts(port, msb_first as HAL_Bool, sample_on_trailing as HAL_Bool, clock_idle_high as HAL_Bool) }
}

pub fn set_spi_chip_select_active_high(port: i32) {
    unsafe { HAL_SetSPIChipSelectActiveHigh(port) }
}

pub fn set_spi_chip_select_active_low(port: i32) {
    unsafe { HAL_SetSPIChipSelectActiveLow(port) }
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
    big_endian: bool
}

pub fn init_spi_accumulator(port: i32, opts: SpiAccumulatorOptions) {
    unsafe { HAL_InitSPIAccumulator() }
}

pub fn free_spi_accumulator(port: i32) {
    unsafe { HAL_FreeSPIAccumulator() }
}

pub fn reset_spi_accumulator(port: i32) {
    unsafe { HAL_ResetSPIAccumulator() }
}

pub fn set_spi_accumulator_center(port: i32, center: i32) {
    unsafe { HAL_SetSPIAccumulatorCenter() }
}

pub fn set_spi_accumulator_deadband(port: i32, deadband: i32) {
    unsafe { HAL_SetSPIAccumulatorDeadband() }
}

pub fn hal_get_spi_accumulator_last_value(port: i32) -> HalResult<i32> {
    hal_call![ ptr HAL_GetSPIAccumulatorLastValue() ]
}

pub fn hal_get_spi_accumulator_value(port: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetSPIAccumulatorValue() ]
}

pub fn hal_get_spi_accumulator_count(port: i32) -> HalResult<i64> {
    hal_call![ ptr HAL_GetSPIAccumulatorCount() ]
}

pub fn hal_get_spi_accumulator_average(port: i32) -> HalResult<f64> {
    hal_call![ ptr HAL_GetSPIAccumulatorAverage() ]
}

pub fn get_spi_accumulator_output(port: i32, value: *mut i64, count: *mut i64) {
    unsafe { HAL_GetSPIAccumulatorOutput(port, &mut value) }
}
