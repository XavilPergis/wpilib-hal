use hal::types::{AnalogInputHandle, AnalogTriggerHandle, NativeBool};
use error::*;
use hal::analog::AnalogInput;
use std::ops::Range;
use std::os::raw::*;

extern "C" {
    fn HAL_InitializeAnalogTrigger(handle: AnalogInputHandle, index: *mut i32, status: *mut i32) -> AnalogTriggerHandle;
    fn HAL_CleanAnalogTrigger(handle: AnalogTriggerHandle, status: *mut i32);
    fn HAL_SetAnalogTriggerLimitsRaw(handle: AnalogTriggerHandle, lower: i32, upper: i32, status: *mut i32);
    fn HAL_SetAnalogTriggerLimitsVoltage(handle: AnalogTriggerHandle, lower: c_double, upper: c_double, status: *mut i32);
    fn HAL_SetAnalogTriggerAveraged(handle: AnalogTriggerHandle, useAveragedValue: NativeBool, status: *mut i32);
    fn HAL_SetAnalogTriggerFiltered(handle: AnalogTriggerHandle, useFilteredValue: NativeBool, status: *mut i32);
    fn HAL_GetAnalogTriggerInWindow(handle: AnalogTriggerHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetAnalogTriggerTriggerState(handle: AnalogTriggerHandle, status: *mut i32) -> NativeBool;
    fn HAL_GetAnalogTriggerOutput(handle: AnalogTriggerHandle, type_: AnalogTriggerType, status: *mut i32) -> NativeBool;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AnalogTriggerType {
  InWindow = 0,
  State = 1,
  RisingPulse = 2,
  FallingPulse = 3,
}

#[derive(Debug)]
pub struct AnalogTrigger<'i> { 
    _input: &'i AnalogInput,
    port: i32,
    index: i32,
}

impl<'i> AnalogTrigger<'i> {
    pub fn new(channel: &'i AnalogInput) -> HalResult<Self> {
        unsafe {
            let mut index = 0;
            let port = hal_call!(HAL_InitializeAnalogTrigger(channel.port, &mut index))?;

            Ok(AnalogTrigger { _input: channel, port, index })
        }
    }

    pub fn set_limits_raw(&self, limits: Range<i32>) -> HalResult<()> {
        // end > start is checked in the HAL
        unsafe { hal_call!(HAL_SetAnalogTriggerLimitsRaw(self.port, limits.start, limits.end)) }
    }

    pub fn set_limits_voltage(&self, limits: Range<f64>) -> HalResult<()> {
        // end > start is checked in the HAL
        unsafe { hal_call!(HAL_SetAnalogTriggerLimitsVoltage(self.port, limits.start, limits.end)) }
    }

    pub fn set_averaged(&self, averaged: bool) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogTriggerAveraged(self.port, averaged as NativeBool)) }
    }

    pub fn set_filtered(&self, filtered: bool) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetAnalogTriggerFiltered(self.port, filtered as NativeBool)) }
    }

    pub fn in_window(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetAnalogTriggerInWindow(self.port)).map(|n| n!= 0) }
    }

    pub fn get_trigger_state(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetAnalogTriggerTriggerState(self.port)).map(|n| n!= 0) }
    }

    pub fn get_trigger_output(&self, trigger_type: AnalogTriggerType) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetAnalogTriggerOutput(self.port, trigger_type)).map(|n| n!= 0) }
    }
}

impl<'i> Drop for AnalogTrigger<'i> {
    fn drop(&mut self) {
        // Ah yes, the pinnacle of API design.
        //
        // void HAL_CleanAnalogTrigger(HAL_AnalogTriggerHandle analogTriggerHandle,
        //                             int32_t* status) {
        //   analogTriggerHandles->Free(analogTriggerHandle);
        //   // caller owns the analog input handle.
        // }
        //
        // Yeah, so the status never gets set, so it's fine to unwrap
        unsafe { hal_call!(HAL_CleanAnalogTrigger(self.port)).unwrap(); }
    }
}
