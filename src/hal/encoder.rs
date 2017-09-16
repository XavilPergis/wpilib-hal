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

pub unsafe fn initialize(source_handle_a: Handle, trigger_type_a: AnalogTriggerType,
                  source_handle_b: Handle, trigger_type_b: AnalogTriggerType,
                  reverse_direction: bool, encoding_type: EncodingType)
                  -> HalResult<EncoderHandle> {
    hal_call!(ptr HAL_InitializeEncoder(source_handle_a, trigger_type_a, source_handle_b, trigger_type_b, reverse_direction as NativeBool, encoding_type))
}

pub unsafe fn free(handle: EncoderHandle) -> HalResult<()> {
    hal_call!(ptr HAL_FreeEncoder(handle))
}

pub unsafe fn get(handle: EncoderHandle) -> HalResult<i32> {
    hal_call!(ptr HAL_GetEncoder(handle))
}

pub unsafe fn get_raw(handle: EncoderHandle) -> HalResult<i32> {
    hal_call!(ptr HAL_GetEncoderRaw(handle))
}

pub unsafe fn get_encoding_scale(handle: EncoderHandle) -> HalResult<i32> {
    hal_call!(ptr HAL_GetEncoderEncodingScale(handle))
}

pub unsafe fn reset(handle: EncoderHandle) -> HalResult<()> {
    hal_call!(ptr HAL_ResetEncoder(handle))
}

pub unsafe fn get_period(handle: EncoderHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_GetEncoderPeriod(handle))
}

pub unsafe fn set_max_period(handle: EncoderHandle, max_period: f64) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderMaxPeriod(handle, max_period))
}

pub unsafe fn get_stopped(handle: EncoderHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetEncoderStopped(handle)).map(|n| n != 0)
}

pub unsafe fn get_direction(handle: EncoderHandle) -> HalResult<bool> {
    hal_call!(ptr HAL_GetEncoderDirection(handle)).map(|n| n != 0)
}

pub unsafe fn get_distance(handle: EncoderHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_GetEncoderDistance(handle))
}

pub unsafe fn get_rate(handle: EncoderHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_GetEncoderRate(handle))
}

pub unsafe fn set_min_rate(handle: EncoderHandle, min_rate: f64) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderMinRate(handle, min_rate))
}

pub unsafe fn set_distance_per_pulse(handle: EncoderHandle, distance_per_pulse: f64) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderDistancePerPulse(handle, distance_per_pulse))
}

pub unsafe fn set_reverse_direction(handle: EncoderHandle, reverse: NativeBool) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderReverseDirection(handle, reverse))
}

pub unsafe fn set_samples_to_average(handle: EncoderHandle, samples_to_average: i32) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderSamplesToAverage(handle, samples_to_average))
}

pub unsafe fn get_samples_to_average(handle: EncoderHandle) -> HalResult<i32> {
    hal_call!(ptr HAL_GetEncoderSamplesToAverage(handle))
}

pub unsafe fn set_index_source(handle: EncoderHandle, digital_source_handle: Handle, trigger_type: AnalogTriggerType, indexing_type: IndexingType) -> HalResult<()> {
    hal_call!(ptr HAL_SetEncoderIndexSource(handle, digital_source_handle, trigger_type, indexing_type))
}

pub unsafe fn get_fpga_index(handle: EncoderHandle) -> HalResult<i32> {
    hal_call!(ptr HAL_GetEncoderFPGAIndex(handle))
}

pub unsafe fn get_decoding_scale_factor(handle: EncoderHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_GetEncoderDecodingScaleFactor(handle))
}

pub unsafe fn get_distance_per_pulse(handle: EncoderHandle) -> HalResult<f64> {
    hal_call!(ptr HAL_GetEncoderDistancePerPulse(handle))
}

pub unsafe fn get_encoding_type(handle: EncoderHandle) -> HalResult<EncodingType> {
    hal_call!(ptr HAL_GetEncodingType(handle)).map(Into::into)
}
