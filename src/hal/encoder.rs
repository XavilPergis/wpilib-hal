use error::*;
use hal::types::{Handle, EncoderHandle, NativeBool};
use hal::analog_trigger::AnalogTriggerType;

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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Encoder {
    pub(crate) handle: Handle,
}

impl Encoder {
    pub fn initialize(source_handle_a: Handle, trigger_type_a: AnalogTriggerType,
                      source_handle_b: Handle, trigger_type_b: AnalogTriggerType,
                      reverse_direction: bool, encoding_type: EncodingType)
                      -> HalResult<Self> {
        unsafe {
            hal_call!(HAL_InitializeEncoder(source_handle_a, trigger_type_a,
                                                source_handle_b, trigger_type_b,
                                                reverse_direction as NativeBool, encoding_type))
                                                .map(|handle| Encoder { handle })
        }
    }

    pub fn free(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_FreeEncoder(self.handle)) }
    }

    pub fn get(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetEncoder(self.handle)) }
    }

    pub fn get_raw(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetEncoderRaw(self.handle)) }
    }

    pub fn get_encoding_scale(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetEncoderEncodingScale(self.handle)) }
    }

    pub fn reset(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_ResetEncoder(self.handle)) }
    }

    pub fn get_period(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetEncoderPeriod(self.handle)) }
    }

    pub fn set_max_period(&self, max_period: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderMaxPeriod(self.handle, max_period)) }
    }

    pub fn get_stopped(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetEncoderStopped(self.handle)).map(|n| n != 0) }
    }

    pub fn get_direction(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetEncoderDirection(self.handle)).map(|n| n != 0) }
    }

    pub fn get_distance(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetEncoderDistance(self.handle)) }
    }

    pub fn get_rate(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetEncoderRate(self.handle)) }
    }

    pub fn set_min_rate(&self, min_rate: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderMinRate(self.handle, min_rate)) }
    }

    pub fn set_distance_per_pulse(&self, distance_per_pulse: f64) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderDistancePerPulse(self.handle, distance_per_pulse)) }
    }

    pub fn set_reverse_direction(&self, reverse: NativeBool) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderReverseDirection(self.handle, reverse)) }
    }

    pub fn set_samples_to_average(&self, samples_to_average: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderSamplesToAverage(self.handle, samples_to_average)) }
    }

    pub fn get_samples_to_average(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetEncoderSamplesToAverage(self.handle)) }
    }

    pub fn set_index_source(&self, digital_source_handle: Handle, trigger_type: AnalogTriggerType, indexing_type: IndexingType) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetEncoderIndexSource(self.handle, digital_source_handle, trigger_type, indexing_type)) }
    }

    pub fn get_fpga_index(&self) -> HalResult<i32> {
        unsafe { hal_call!(HAL_GetEncoderFPGAIndex(self.handle)) }
    }

    pub fn get_decoding_scale_factor(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetEncoderDecodingScaleFactor(self.handle)) }
    }

    pub fn get_distance_per_pulse(&self) -> HalResult<f64> {
        unsafe { hal_call!(HAL_GetEncoderDistancePerPulse(self.handle)) }
    }

    pub fn get_encoding_type(&self) -> HalResult<EncodingType> {
        unsafe { hal_call!(HAL_GetEncodingType(self.handle)).map(Into::into) }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        // AGAIN, another unused status parameter
        unsafe { HAL_FreeEncoder(self.handle, ::std::ptr::null_mut()); }
    }
}
