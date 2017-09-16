use ::error::*;
use ::raw::HAL_Bool;

extern "C" {
    pub fn HAL_InitializeSPI(port: SpiPort, status: *mut i32);
    pub fn HAL_TransactionSPI(port: SpiPort, dataToSend: *const u8,
                              dataReceived: *mut u8, size: i32) -> i32;
    pub fn HAL_WriteSPI(port: SpiPort, dataToSend: *const u8, sendSize: i32) -> i32;
    pub fn HAL_ReadSPI(port: SpiPort, buffer: *mut u8, count: i32) -> i32;
    pub fn HAL_CloseSPI(port: SpiPort);
    pub fn HAL_SetSPISpeed(port: SpiPort, speed: i32);
    pub fn HAL_SetSPIOpts(port: SpiPort, msbFirst: HAL_Bool,
                          sampleOnTrailing: HAL_Bool, clkIdleHigh: HAL_Bool);
    pub fn HAL_SetSPIChipSelectActiveHigh(port: SpiPort, status: *mut i32);
    pub fn HAL_SetSPIChipSelectActiveLow(port: SpiPort, status: *mut i32);
    pub fn HAL_GetSPIHandle(port: SpiPort) -> i32;
    pub fn HAL_SetSPIHandle(port: SpiPort, handle: i32);
    pub fn HAL_InitSPIAccumulator(port: SpiPort, period: i32, cmd: i32,
                                  xferSize: i32, validMask: i32,
                                  validValue: i32, dataShift: i32,
                                  dataSize: i32, isSigned: HAL_Bool,
                                  bigEndian: HAL_Bool, status: *mut i32);
    pub fn HAL_FreeSPIAccumulator(port: SpiPort, status: *mut i32);
    pub fn HAL_ResetSPIAccumulator(port: SpiPort, status: *mut i32);
    pub fn HAL_SetSPIAccumulatorCenter(port: SpiPort, center: i32,
                                       status: *mut i32);
    pub fn HAL_SetSPIAccumulatorDeadband(port: SpiPort, deadband: i32,
                                         status: *mut i32);
    pub fn HAL_GetSPIAccumulatorLastValue(port: SpiPort, status: *mut i32) -> i32;
    pub fn HAL_GetSPIAccumulatorValue(port: SpiPort, status: *mut i32) -> i64;
    pub fn HAL_GetSPIAccumulatorCount(port: SpiPort, status: *mut i32) -> i64;
    pub fn HAL_GetSPIAccumulatorAverage(port: SpiPort, status: *mut i32)
     -> ::std::os::raw::c_double;
    pub fn HAL_GetSPIAccumulatorOutput(port: SpiPort, value: *mut i64,
                                       count: *mut i64, status: *mut i32);
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SpiPort {
    CS0 = 0,
    CS1,
    CS2,
    CS3,
    MXP,
}

pub unsafe fn initialize(port: SpiPort) -> HalResult<()> {
    hal_call!(ptr HAL_InitializeSPI(port))
}

// NOTE: size must be in the range [0,7]
pub unsafe fn transaction(port: SpiPort, send_buffer: &[u8], recv_buffer: &mut [u8], size: i32) -> i32 {
    HAL_TransactionSPI(port, send_buffer.as_ptr(), recv_buffer.as_mut_ptr(), size)
}

pub fn write(port: SpiPort, buffer: &[u8]) -> i32 {
    unsafe { HAL_WriteSPI(port, buffer.as_ptr(), buffer.len() as i32) }
}

pub fn read(port: SpiPort, buffer: &mut [u8]) -> i32 {
    unsafe { HAL_ReadSPI(port, buffer.as_mut_ptr(), buffer.len() as i32) }
}

pub unsafe fn close(port: SpiPort) {
    HAL_CloseSPI(port)
}

pub unsafe fn set_speed(port: SpiPort, speed: i32) {
    HAL_SetSPISpeed(port, speed)
}

pub unsafe fn set_opts(port: SpiPort, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
    HAL_SetSPIOpts(port,
                   msb_first as HAL_Bool,
                   sample_on_trailing as HAL_Bool,
                   clock_idle_high as HAL_Bool)
}

pub unsafe fn set_chip_select_active_high(port: SpiPort) -> HalResult<()> {
    hal_call!(ptr HAL_SetSPIChipSelectActiveHigh(port))
}

pub unsafe fn set_chip_select_active_low(port: SpiPort) -> HalResult<()> {
    hal_call!(ptr HAL_SetSPIChipSelectActiveLow(port))
}

pub unsafe fn get_handle(port: SpiPort) -> i32 {
    HAL_GetSPIHandle(port)
}

pub unsafe fn set_handle(port: SpiPort, handle: i32) {
    HAL_SetSPIHandle(port, handle)
}

pub unsafe fn init_accumulator(port: SpiPort,
                               period: i32, 
                               cmd: i32,
                               transfer_size: i32,
                               valid_mask: i32,
                               valid_value: i32,
                               data_shift: i32,
                               data_size: i32,
                               is_signed: bool, 
                               big_endian: bool)
                               -> HalResult<()> {
    hal_call!(ptr HAL_InitSPIAccumulator(port, 
              period, 
              cmd, 
              transfer_size, 
              valid_mask, 
              valid_value, 
              data_shift, 
              data_size, 
              is_signed as HAL_Bool, 
              big_endian as HAL_Bool))
}

pub fn free_accumulator(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_FreeSPIAccumulator(port)) }
}

pub fn reset_accumulator(port: SpiPort) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ResetSPIAccumulator(port)) }
}

pub fn set_accumulator_center(port: SpiPort, center: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetSPIAccumulatorCenter(port, center)) }
}

pub fn set_accumulator_deadband(port: SpiPort, deadband: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetSPIAccumulatorDeadband(port, deadband)) }
}

pub fn hal_get_accumulator_last_value(port: SpiPort) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetSPIAccumulatorLastValue(port)) }
}

pub fn hal_get_accumulator_value(port: SpiPort) -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetSPIAccumulatorValue(port)) }
}

pub fn hal_get_accumulator_count(port: SpiPort) -> HalResult<i64> {
    unsafe { hal_call!(ptr HAL_GetSPIAccumulatorCount(port)) }
}

pub fn hal_get_accumulator_average(port: SpiPort) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetSPIAccumulatorAverage(port)) }
}

/// value, count
pub fn get_accumulator_output(port: SpiPort) -> HalResult<(i64, i64)> {
    unsafe {
        let mut value = 0;
        let mut count = 0;
        hal_call!(ptr HAL_GetSPIAccumulatorOutput(port, &mut value, &mut count))?;
        Ok((value, count))
    }
}
