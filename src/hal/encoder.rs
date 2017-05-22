use ::error::*;
use ::raw::*;
use handle::*;
use hal::analog_trigger::AnalogTriggerType;

pub type RawIndexingType = HAL_EncoderIndexingType;
pub type RawEncodingType = HAL_EncoderEncodingType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IndexingType {
    ResetWhileHigh,
    ResetWhileLow,
    ResetOnFallingEdge,
    ResetOnRisingEdge,
}

impl_convert! {
    HAL_EncoderIndexingType, IndexingType;
    HAL_kResetWhileHigh <=> ResetWhileHigh,
    HAL_kResetWhileLow <=> ResetWhileLow,
    HAL_kResetOnFallingEdge <=> ResetOnFallingEdge,
    HAL_kResetOnRisingEdge <=> ResetOnRisingEdge
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EncodingType {
    Encoder1X,
    Encoder2X,
    Encoder4X,
}

impl_convert! {
    HAL_EncoderEncodingType, EncodingType;
    HAL_Encoder_k1X <=> Encoder1X,
    HAL_Encoder_k2X <=> Encoder2X,
    HAL_Encoder_k4X <=> Encoder4X
}

pub unsafe fn initialize(source_handle_a: Handle, trigger_type_a: AnalogTriggerType,
                  source_handle_b: Handle, trigger_type_b: AnalogTriggerType,
                  reverse_direction: bool, encoding_type: EncodingType)
                  -> HalResult<EncoderHandle> {
    hal_call![ ptr HAL_InitializeEncoder(source_handle_a, trigger_type_a.into(), source_handle_b, trigger_type_b.into(), reverse_direction as HAL_Bool, encoding_type.into()) ]
}

pub unsafe fn free(handle: EncoderHandle) -> HalResult<()> {
    hal_call![ ptr HAL_FreeEncoder(handle) ]
}

pub unsafe fn get(handle: EncoderHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetEncoder(handle) ]
}

pub unsafe fn get_raw(handle: EncoderHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetEncoderRaw(handle) ]
}

pub unsafe fn get_encoding_scale(handle: EncoderHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetEncoderEncodingScale(handle) ]
}

pub unsafe fn reset(handle: EncoderHandle) -> HalResult<()> {
    hal_call![ ptr HAL_ResetEncoder(handle) ]
}

pub unsafe fn get_period(handle: EncoderHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetEncoderPeriod(handle) ]
}

pub unsafe fn set_max_period(handle: EncoderHandle, max_period: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderMaxPeriod(handle, max_period) ]
}

pub unsafe fn get_stopped(handle: EncoderHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetEncoderStopped(handle) ].map(|n| n != 0)
}

pub unsafe fn get_direction(handle: EncoderHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetEncoderDirection(handle) ].map(|n| n != 0)
}

pub unsafe fn get_distance(handle: EncoderHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetEncoderDistance(handle) ]
}

pub unsafe fn get_rate(handle: EncoderHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetEncoderRate(handle) ]
}

pub unsafe fn set_min_rate(handle: EncoderHandle, min_rate: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderMinRate(handle, min_rate) ]
}

pub unsafe fn set_distance_per_pulse(handle: EncoderHandle, distance_per_pulse: f64) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderDistancePerPulse(handle, distance_per_pulse) ]
}

pub unsafe fn set_reverse_direction(handle: EncoderHandle, reverse: HAL_Bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderReverseDirection(handle, reverse) ]
}

pub unsafe fn set_samples_to_average(handle: EncoderHandle, samples_to_average: i32) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderSamplesToAverage(handle, samples_to_average) ]
}

pub unsafe fn get_samples_to_average(handle: EncoderHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetEncoderSamplesToAverage(handle) ]
}

pub unsafe fn set_index_source(handle: EncoderHandle, digital_source_handle: Handle, trigger_type: AnalogTriggerType, indexing_type: IndexingType) -> HalResult<()> {
    hal_call![ ptr HAL_SetEncoderIndexSource(handle, digital_source_handle, trigger_type.into(), indexing_type.into()) ]
}

pub unsafe fn get_fpga_index(handle: EncoderHandle) -> HalResult<i32> {
    hal_call![ ptr HAL_GetEncoderFPGAIndex(handle) ]
}

pub unsafe fn get_decoding_scale_factor(handle: EncoderHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetEncoderDecodingScaleFactor(handle) ]
}

pub unsafe fn get_distance_per_pulse(handle: EncoderHandle) -> HalResult<f64> {
    hal_call![ ptr HAL_GetEncoderDistancePerPulse(handle) ]
}

pub unsafe fn get_encoding_type(handle: EncoderHandle) -> HalResult<EncodingType> {
    hal_call![ ptr HAL_GetEncoderEncodingType(handle) ].map(Into::into)
}
