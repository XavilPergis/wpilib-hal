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

impl From<RawIndexingType> for IndexingType {
    fn from(raw: RawIndexingType) -> IndexingType {
        match raw {
            HAL_EncoderIndexingType::HAL_kResetWhileHigh => IndexingType::ResetWhileHigh,
            HAL_EncoderIndexingType::HAL_kResetWhileLow => IndexingType::ResetWhileLow,
            HAL_EncoderIndexingType::HAL_kResetOnFallingEdge => IndexingType::ResetOnFallingEdge,
            HAL_EncoderIndexingType::HAL_kResetOnRisingEdge => IndexingType::ResetOnRisingEdge,
        }
    }
}

impl From<IndexingType> for RawIndexingType {
    fn from(idt: IndexingType) -> RawIndexingType {
        match idt {
            IndexingType::ResetWhileHigh => HAL_EncoderIndexingType::HAL_kResetWhileHigh,
            IndexingType::ResetWhileLow => HAL_EncoderIndexingType::HAL_kResetWhileLow,
            IndexingType::ResetOnFallingEdge => HAL_EncoderIndexingType::HAL_kResetOnFallingEdge,
            IndexingType::ResetOnRisingEdge => HAL_EncoderIndexingType::HAL_kResetOnRisingEdge,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EncodingType {
    Encoder1X,
    Encoder2X,
    Encoder4X,
}

impl From<RawEncodingType> for EncodingType {
    fn from(raw: RawEncodingType) -> EncodingType {
        match raw {
            HAL_EncoderEncodingType::HAL_Encoder_k1X => EncodingType::Encoder1X,
            HAL_EncoderEncodingType::HAL_Encoder_k2X => EncodingType::Encoder2X,
            HAL_EncoderEncodingType::HAL_Encoder_k4X => EncodingType::Encoder4X,
        }
    }
}

impl From<EncodingType> for RawEncodingType {
    fn from(et: EncodingType) -> RawEncodingType {
        match et {
            EncodingType::Encoder1X => HAL_EncoderEncodingType::HAL_Encoder_k1X,
            EncodingType::Encoder2X => HAL_EncoderEncodingType::HAL_Encoder_k2X,
            EncodingType::Encoder4X => HAL_EncoderEncodingType::HAL_Encoder_k4X,
        }
    }
}

pub fn initialize(source_handle_a: Handle, trigger_type_a: AnalogTriggerType,
                  source_handle_b: Handle, trigger_type_b: AnalogTriggerType,
                  reverse_direction: bool, encoding_type: EncodingType)
                  -> HalResult<EncoderHandle> {
    unsafe { hal_call![ ptr HAL_InitializeEncoder(source_handle_a, trigger_type_a.into_raw(), source_handle_b, trigger_type_b.into_raw(), reverse_direction as HAL_Bool, encoding_type.into()) ] }
}

pub fn free(handle: EncoderHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_FreeEncoder(handle) ] }
}

pub fn get(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetEncoder(handle) ] }
}

pub fn get_raw(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetEncoderRaw(handle) ] }
}

pub fn get_encoding_scale(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetEncoderEncodingScale(handle) ] }
}

pub fn reset(handle: EncoderHandle) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_ResetEncoder(handle) ] }
}

pub fn get_period(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetEncoderPeriod(handle) ] }
}

pub fn set_max_period(handle: EncoderHandle, max_period: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderMaxPeriod(handle, max_period) ] }
}

pub fn get_stopped(handle: EncoderHandle) -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetEncoderStopped(handle) ].map(|n| n != 0) }
}

pub fn get_direction(handle: EncoderHandle) -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetEncoderDirection(handle) ].map(|n| n != 0) }
}

pub fn get_distance(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetEncoderDistance(handle) ] }
}

pub fn get_rate(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetEncoderRate(handle) ] }
}

pub fn set_min_rate(handle: EncoderHandle, min_rate: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderMinRate(handle, min_rate) ] }
}

pub fn set_distance_per_pulse(handle: EncoderHandle, distance_per_pulse: f64) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderDistancePerPulse(handle, distance_per_pulse) ] }
}

pub fn set_reverse_direction(handle: EncoderHandle, reverse: HAL_Bool) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderReverseDirection(handle, reverse) ] }
}

pub fn set_samples_to_average(handle: EncoderHandle, samples_to_average: i32) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderSamplesToAverage(handle, samples_to_average) ] }
}

pub fn get_samples_to_average(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetEncoderSamplesToAverage(handle) ] }
}

pub fn set_index_source(handle: EncoderHandle, digital_source_handle: Handle, trigger_type: AnalogTriggerType, indexing_type: IndexingType) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetEncoderIndexSource(handle, digital_source_handle, trigger_type.into_raw(), indexing_type.into()) ] }
}

pub fn get_fpga_index(handle: EncoderHandle) -> HalResult<i32> {
    unsafe { hal_call![ ptr HAL_GetEncoderFPGAIndex(handle) ] }
}

pub fn get_decoding_scale_factor(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetEncoderDecodingScaleFactor(handle) ] }
}

pub fn get_distance_per_pulse(handle: EncoderHandle) -> HalResult<f64> {
    unsafe { hal_call![ ptr HAL_GetEncoderDistancePerPulse(handle) ] }
}

pub fn get_encoding_type(handle: EncoderHandle) -> HalResult<EncodingType> {
    unsafe { hal_call![ ptr HAL_GetEncoderEncodingType(handle) ].map(Into::into) }
}
