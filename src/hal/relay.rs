use ::error::*;
use hal::handle::*;
use ::raw::*;

// FIXME
pub fn initialize(handle: PortHandle, fwd: bool) -> HalResult<RelayHandle> {
    if self::check_channel(handle) {
        unsafe { hal_call![ ptr HAL_InitializeRelayPort(handle, fwd as HAL_Bool) ] }
    } else {
        Err(HalError::BadChannelType)
    }
}

pub fn free(handle: RelayHandle) {
    unsafe { HAL_FreeRelayPort(handle) }
}

pub fn check_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

pub fn set_active(handle: RelayHandle, on: bool) -> HalResult<()> {
    unsafe { hal_call![ ptr HAL_SetRelay(handle, on as HAL_Bool) ] }
}

pub fn get_active(handle: RelayHandle) -> HalResult<bool> {
    unsafe { hal_call![ ptr HAL_GetRelay(handle) ].map(|n| n != 0) }
}
