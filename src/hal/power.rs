use ::error::*;
use ::raw::*;

pub enum Something {
    Input, User6v, User5v, User3v3
}

pub unsafe fn get_voltage(something: Something) -> HalResult<f64> {
    match something {
        Something::Input => get_vin_voltage(),
        Something::User6v => get_user_voltage6v(),
        Something::User5v => get_user_voltage5v(),
        Something::User3v3 => get_user_voltage3v3()
    }
}

pub unsafe fn get_current(something: Something) -> HalResult<f64> {
    match something {
        Something::Input => get_vin_current(),
        Something::User6v => get_user_current6v(),
        Something::User5v => get_user_current5v(),
        Something::User3v3 => get_user_current3v3()
    }
}

pub unsafe fn get_vin_voltage() -> HalResult<f64> {
    hal_call![ ptr HAL_GetVinVoltage() ]
}

pub unsafe fn get_vin_current() -> HalResult<f64> {
    hal_call![ ptr HAL_GetVinCurrent() ]
}

pub unsafe fn get_user_voltage6v() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserVoltage6V() ]
}

pub unsafe fn get_user_current6v() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserCurrent6V() ]
}

pub unsafe fn get_user_active6v() -> HalResult<bool> {
    hal_call![ ptr HAL_GetUserActive6V() ].map(|n| n != 0)
}

pub unsafe fn get_user_current_faults6v() -> HalResult<i32> {
    hal_call![ ptr HAL_GetUserCurrentFaults6V() ]
}

pub unsafe fn get_user_voltage5v() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserVoltage5V() ]
}

pub unsafe fn get_user_current5v() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserCurrent5V() ]
}

pub unsafe fn get_user_active5v() -> HalResult<bool> {
    hal_call![ ptr HAL_GetUserActive5V() ].map(|n| n != 0)
}

pub unsafe fn get_user_current_faults5v() -> HalResult<i32> {
    hal_call![ ptr HAL_GetUserCurrentFaults5V() ]
}

pub unsafe fn get_user_voltage3v3() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserVoltage3V3() ]
}

pub unsafe fn get_user_current3v3() -> HalResult<f64> {
    hal_call![ ptr HAL_GetUserCurrent3V3() ]
}

pub unsafe fn get_user_active3v3() -> HalResult<bool> {
    hal_call![ ptr HAL_GetUserActive3V3() ].map(|n| n != 0)
}

pub unsafe fn get_user_current_faults3v3() -> HalResult<i32> {
    hal_call![ ptr HAL_GetUserCurrentFaults3V3() ]
}
