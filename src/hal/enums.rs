use raw::*;

pub type RawAnalogTriggerType = HAL_AnalogTriggerType;
pub type RawCounterMode = HAL_Counter_Mode;
pub type RawRuntimeType = HAL_RuntimeType;
pub type RawAccelerometerRange = HAL_AccelerometerRange;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalogTriggerType {
    InWindow, State, RisingPulse, FallingPulse
}

impl From<RawAnalogTriggerType> for AnalogTriggerType {
    fn from(raw: RawAnalogTriggerType) -> Self {
        match raw {
            RawAnalogTriggerType::HAL_Trigger_kInWindow => AnalogTriggerType::InWindow,
            RawAnalogTriggerType::HAL_Trigger_kState => AnalogTriggerType::State,
            RawAnalogTriggerType::HAL_Trigger_kRisingPulse => AnalogTriggerType::RisingPulse,
            RawAnalogTriggerType::HAL_Trigger_kFallingPulse => AnalogTriggerType::FallingPulse
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CounterMode {
    TwoPulse, Semiperiod, PulseLength, ExternalDirection
}

impl From<RawCounterMode> for CounterMode {
    fn from(raw: RawCounterMode) -> Self {
        match raw {
            RawCounterMode::HAL_Counter_kExternalDirection => CounterMode::ExternalDirection,
            RawCounterMode::HAL_Counter_kPulseLength => CounterMode::PulseLength,
            RawCounterMode::HAL_Counter_kSemiperiod => CounterMode::Semiperiod,
            RawCounterMode::HAL_Counter_kTwoPulse => CounterMode::TwoPulse
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RuntimeType {
    Native, Mock
}

impl From<RawRuntimeType> for RuntimeType {
    fn from(raw: RawRuntimeType) -> Self {
        match raw {
            RawRuntimeType::HAL_Athena => RuntimeType::Native,
            RawRuntimeType::HAL_Mock => RuntimeType::Mock,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AccelerometerRange {
    AccelerometerRange2G,
    AccelerometerRange4G,
    AccelerometerRange8G
}

impl From<RawAccelerometerRange> for AccelerometerRange {
    fn from(raw: RawAccelerometerRange) -> Self {
        match raw {
            RawAccelerometerRange::HAL_AccelerometerRange_k2G => AccelerometerRange::AccelerometerRange2G,
            RawAccelerometerRange::HAL_AccelerometerRange_k4G => AccelerometerRange::AccelerometerRange4G,
            RawAccelerometerRange::HAL_AccelerometerRange_k8G => AccelerometerRange::AccelerometerRange8G,
        }
    }
}
