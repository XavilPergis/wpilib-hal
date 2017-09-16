use error::*;
use hal::types::{PortHandle, RelayHandle, NativeBool};

extern "C" {
    fn HAL_InitializeRelayPort(portHandle: PortHandle, fwd: NativeBool, status: *mut i32) -> RelayHandle;
    fn HAL_FreeRelayPort(relayPortHandle: RelayHandle);
    fn HAL_CheckRelayChannel(channel: i32) -> NativeBool;
    fn HAL_SetRelay(relayPortHandle: RelayHandle, on: NativeBool, status: *mut i32);
    fn HAL_GetRelay(relayPortHandle: RelayHandle, status: *mut i32) -> NativeBool;
}


pub fn initialize(handle: PortHandle, fwd: bool) -> HalResult<RelayHandle> {
    unsafe { hal_call!(ptr HAL_InitializeRelayPort(handle, fwd as NativeBool)) }
}

pub fn free(handle: RelayHandle) {
    unsafe { HAL_FreeRelayPort(handle) }
}

pub fn check_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

pub fn set_active(handle: RelayHandle, on: bool) -> HalResult<()> {
    unsafe { hal_call!(ptr HAL_SetRelay(handle, on as NativeBool)) }
}

pub fn get_active(handle: RelayHandle) -> HalResult<bool> {
    unsafe { hal_call!(ptr HAL_GetRelay(handle)).map(|n| n != 0) }
}
