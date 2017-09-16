use error::*;
use hal::types::{Handle, EncoderHandle, NativeBool};
use hal::analog::AnalogTriggerType;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IndexingType {
    ResetWhileHigh = 0,
    ResetWhileLow = 1,
    ResetOnFallingEdge = 2,
    ResetOnRisingEdge = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum EncodingType {
    Encoder1X = 0,
    Encoder2X = 1,
    Encoder4X = 2,
}

extern "C" {
    pub fn HAL_InitializeEncoder(digitalSourceHandleA: Handle,
                                 analogTriggerTypeA: AnalogTriggerType,
                                 digitalSourceHandleB: Handle,
                                 analogTriggerTypeB: AnalogTriggerType,
                                 reverseDirection: NativeBool,
                                 encodingType: EncodingType,
                                 status: *mut i32) -> EncoderHandle;
    pub fn HAL_FreeEncoder(encoderHandle: EncoderHandle, status: *mut i32);
    pub fn HAL_GetEncoder(encoderHandle: EncoderHandle, status: *mut i32) -> i32;
    pub fn HAL_GetEncoderRaw(encoderHandle: EncoderHandle, status: *mut i32) -> i32;
    pub fn HAL_GetEncoderEncodingScale(encoderHandle: EncoderHandle, status: *mut i32) -> i32;
    pub fn HAL_ResetEncoder(encoderHandle: EncoderHandle, status: *mut i32);
    pub fn HAL_GetEncoderPeriod(encoderHandle: EncoderHandle, status: *mut i32) -> ::std::os::raw::c_double;
    pub fn HAL_SetEncoderMaxPeriod(encoderHandle: EncoderHandle, maxPeriod: ::std::os::raw::c_double, status: *mut i32);
    pub fn HAL_GetEncoderStopped(encoderHandle: EncoderHandle, status: *mut i32) -> NativeBool;
    pub fn HAL_GetEncoderDirection(encoderHandle: EncoderHandle, status: *mut i32) -> NativeBool;
    pub fn HAL_GetEncoderDistance(encoderHandle: EncoderHandle, status: *mut i32) -> ::std::os::raw::c_double;
    pub fn HAL_GetEncoderRate(encoderHandle: EncoderHandle, status: *mut i32) -> ::std::os::raw::c_double;
    pub fn HAL_SetEncoderMinRate(encoderHandle: EncoderHandle, minRate: ::std::os::raw::c_double, status: *mut i32);
    pub fn HAL_SetEncoderDistancePerPulse(encoderHandle: EncoderHandle,
                                          distancePerPulse:
                                              ::std::os::raw::c_double,
                                          status: *mut i32);
    pub fn HAL_SetEncoderReverseDirection(encoderHandle: EncoderHandle, reverseDirection: NativeBool, status: *mut i32);
    pub fn HAL_SetEncoderSamplesToAverage(encoderHandle: EncoderHandle, samplesToAverage: i32, status: *mut i32);
    pub fn HAL_GetEncoderSamplesToAverage(encoderHandle: EncoderHandle, status: *mut i32) -> i32;
    pub fn HAL_SetEncoderIndexSource(encoderHandle: EncoderHandle,
                                     digitalSourceHandle: Handle,
                                     analogTriggerType: AnalogTriggerType,
                                     type_: IndexingType,
                                     status: *mut i32);
    pub fn HAL_GetEncoderFPGAIndex(encoderHandle: EncoderHandle, status: *mut i32) -> i32;
    pub fn HAL_GetEncoderDecodingScaleFactor(encoderHandle: EncoderHandle, status: *mut i32) -> ::std::os::raw::c_double;
    pub fn HAL_GetEncoderDistancePerPulse(encoderHandle: EncoderHandle, status: *mut i32) -> ::std::os::raw::c_double;
    pub fn HAL_GetEncodingType(encoderHandle: EncoderHandle, status: *mut i32) -> EncodingType;
}

#[inline(always)]
pub fn initialize(source_handle_a: Handle, trigger_type_a: AnalogTriggerType,
                  source_handle_b: Handle, trigger_type_b: AnalogTriggerType,
                  reverse_direction: bool, encoding_type: EncodingType)
                  -> HalResult<EncoderHandle> {
    unsafe { hal_call!(ptr HAL_InitializeEncoder(source_handle_a, trigger_type_a, source_handle_b, trigger_type_b, reverse_direction as NativeBool, encoding_type)) }
}

#[inline(always)]
pub fn free(handle: EncoderHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_FreeEncoder(handle)) }
}

#[inline(always)]
pub fn get(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetEncoder(handle)) }
}

#[inline(always)]
pub fn get_raw(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetEncoderRaw(handle)) }
}

#[inline(always)]
pub fn get_encoding_scale(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetEncoderEncodingScale(handle)) }
}

#[inline(always)]
pub fn reset(handle: EncoderHandle) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_ResetEncoder(handle)) }
}

#[inline(always)]
pub fn get_period(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetEncoderPeriod(handle)) }
}

#[inline(always)]
pub fn set_max_period(handle: EncoderHandle, max_period: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderMaxPeriod(handle, max_period)) }
}

#[inline(always)]
pub fn get_stopped(handle: EncoderHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetEncoderStopped(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_direction(handle: EncoderHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetEncoderDirection(handle)).map(|n| n != 0) }
}

#[inline(always)]
pub fn get_distance(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetEncoderDistance(handle)) }
}

#[inline(always)]
pub fn get_rate(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetEncoderRate(handle)) }
}

#[inline(always)]
pub fn set_min_rate(handle: EncoderHandle, min_rate: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderMinRate(handle, min_rate)) }
}

#[inline(always)]
pub fn set_distance_per_pulse(handle: EncoderHandle, distance_per_pulse: f64) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderDistancePerPulse(handle, distance_per_pulse)) }
}

#[inline(always)]
pub fn set_reverse_direction(handle: EncoderHandle, reverse: NativeBool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderReverseDirection(handle, reverse)) }
}

#[inline(always)]
pub fn set_samples_to_average(handle: EncoderHandle, samples_to_average: i32) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderSamplesToAverage(handle, samples_to_average)) }
}

#[inline(always)]
pub fn get_samples_to_average(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetEncoderSamplesToAverage(handle)) }
}

#[inline(always)]
pub fn set_index_source(handle: EncoderHandle, digital_source_handle: Handle, trigger_type: AnalogTriggerType, indexing_type: IndexingType) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetEncoderIndexSource(handle, digital_source_handle, trigger_type, indexing_type)) }
}

#[inline(always)]
pub fn get_fpga_index(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call!(ptr HAL_GetEncoderFPGAIndex(handle)) }
}

#[inline(always)]
pub fn get_decoding_scale_factor(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetEncoderDecodingScaleFactor(handle)) }
}

#[inline(always)]
pub fn get_distance_per_pulse(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call!(ptr HAL_GetEncoderDistancePerPulse(handle)) }
}

#[inline(always)]
pub fn get_encoding_type(handle: EncoderHandle) -> HalResult<EncodingType> {
    unsafe { hal_call!(ptr HAL_GetEncodingType(handle)).map(Into::into) }
}
