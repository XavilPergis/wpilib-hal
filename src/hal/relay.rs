use ::error::*;
use hal::handle::*;
use ::raw::*;

// FIXME
pub unsafe fn initialize(handle: PortHandle, fwd: bool) -> HalResult<RelayHandle> {
    if self::check_channel(handle) {
        hal_call![ ptr HAL_InitializeRelayPort(handle, fwd as HAL_Bool) ]
    } else {
        Err(HalError::BadChannelType)
    }
}

pub unsafe fn free(handle: RelayHandle) {
    HAL_FreeRelayPort(handle)
}

pub unsafe fn check_channel(channel: i32) -> bool {
    HAL_CheckRelayChannel(channel) != 0
}

pub unsafe fn set_active(handle: RelayHandle, on: bool) -> HalResult<()> {
    hal_call![ ptr HAL_SetRelay(handle, on as HAL_Bool) ]
}

pub unsafe fn get_active(handle: RelayHandle) -> HalResult<bool> {
    hal_call![ ptr HAL_GetRelay(handle) ].map(|n| n != 0)
}
