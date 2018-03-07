use error::*;
use hal::types::{PortHandle, SolenoidHandle, NativeBool};

extern "C" {
    fn HAL_InitializeSolenoidPort(handle: PortHandle, status: *mut i32) -> SolenoidHandle;
    fn HAL_FreeSolenoidPort(handle: SolenoidHandle);
    fn HAL_CheckSolenoidModule(module: i32) -> NativeBool;
    fn HAL_CheckSolenoidChannel(channel: i32) -> NativeBool;
    fn HAL_GetSolenoid(handle: SolenoidHandle, status: *mut i32) -> NativeBool;
    fn HAL_SetSolenoid(handle: SolenoidHandle, value: NativeBool, status: *mut i32);
    fn HAL_GetPCMSolenoidBlackList(module: i32, status: *mut i32) -> i32;
    fn HAL_SetOneShotDuration(handle: SolenoidHandle, duration_ms: i32, status: *mut i32);
    fn HAL_FireOneShot(handle: SolenoidHandle, status: *mut i32);

    // TODO: unused
    fn HAL_GetAllSolenoids(module: i32, status: *mut i32) -> i32;
    fn HAL_SetAllSolenoids(module: i32, state: i32, status: *mut i32);
    fn HAL_GetPCMSolenoidVoltageStickyFault(module: i32, status: *mut i32) -> NativeBool;
    fn HAL_GetPCMSolenoidVoltageFault(module: i32, status: *mut i32) -> NativeBool;
    fn HAL_ClearAllPCMStickyFaults(module: i32, status: *mut i32);
}

fn check_module(module: i32) -> bool { unsafe { HAL_CheckSolenoidModule(module) != 0 } }
fn check_channel(channel: i32) -> bool { unsafe { HAL_CheckSolenoidChannel(channel) != 0 } }

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Solenoid {
    channel: i32,
    handle: i32,
}

impl Solenoid {
    pub fn new(channel: i32) -> HalResult<Self> {
        // TODO
        Solenoid::with_module(0, channel)
    }

    pub fn with_module(module: i32, channel: i32) -> HalResult<Self> {
        if !check_channel(channel) { return Err(HalError::InvalidChannel(channel)); }
        if !check_module(module) { return Err(HalError::InvalidModule(module)); }

        // Will this ever return an invalid handle after validating the channel/module beforehand?
        let port_handle = ::hal::get_port_with_module(module, channel).ok_or(HalError::InvalidChannel(channel))?;
        let handle = unsafe { hal_call!(HAL_InitializeSolenoidPort(port_handle))? };

        Ok(Solenoid { channel, handle })
    }

    pub fn set(&self, on: bool) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetSolenoid(self.handle, on as NativeBool)) }
    }

    pub fn get(&self) -> HalResult<bool> {
        unsafe { hal_call!(HAL_GetSolenoid(self.handle)).map(|n| n != 0) }
    }

    /// Check if this solenoid is blacklisted. If a solenoid is shorted, it is added to a
    /// blacklist, and is disabled until faults are cleared or a power cycle.
    pub fn is_blacklisted(&self, module: i32) -> HalResult<bool> {
        unsafe {
            // returns a "list" of bools packed into an int, with the LSB being index 0
            hal_call!(HAL_GetPCMSolenoidBlackList(module))
                .map(|blacklist_bits| blacklist_bits & (1 << self.channel) != 0)
        }
    }

    pub fn set_pulse_duration(&self, duration: i32) -> HalResult<()> {
        unsafe { hal_call!(HAL_SetOneShotDuration(self.handle, duration)) }
    }

    pub fn fire_pulse(&self) -> HalResult<()> {
        unsafe { hal_call!(HAL_FireOneShot(self.handle)) }
    }
}

impl Drop for Solenoid {
    fn drop(&mut self) {
        unsafe { HAL_FreeSolenoidPort(self.handle) }
    }
}
