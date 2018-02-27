use error::*;
use hal::types::{Handle, PortHandle, RelayHandle, NativeBool};

extern "C" {
    fn HAL_InitializeRelayPort(portHandle: PortHandle, fwd: NativeBool, status: *mut i32) -> RelayHandle;
    fn HAL_FreeRelayPort(relayPortHandle: RelayHandle);
    fn HAL_CheckRelayChannel(channel: i32) -> NativeBool;
    fn HAL_SetRelay(relayPortHandle: RelayHandle, on: NativeBool, status: *mut i32);
    fn HAL_GetRelay(relayPortHandle: RelayHandle, status: *mut i32) -> NativeBool;
}

/// A raw relay.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Relay {
    pub(crate) handle: Handle
}

fn check_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

impl Relay {
    pub fn new(port: i32, forward: bool) -> HalResult<Self> {
        if !check_channel(port) { return Err(HalError::OutOfRange); }
        let handle = unsafe { hal_call!(ptr HAL_InitializeRelayPort(port, forward))? };
        Ok(Relay { handle })
    }

    pub fn set(&self, active: bool) -> HalResult<()> {
        unsafe { hal_call!(ptr HAL_SetRelay(self.handle, active as NativeBool)) }
    }

    pub fn get(&self) -> HalResult<bool> {
        unsafe { hal_call!(ptr HAL_GetRelay(self.handle)).map(|n| n != 0) }
    }
}

impl Drop for Relay {
    fn drop(&mut self) {
        unsafe { HAL_FreeRelayPort(self.handle); }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum DualValue {
    On, Off, Forward, Reverse,
}

/// A relay that has both forward and backward modes
#[derive(Debug, Eq, PartialEq, Hash)]
struct DoubleRelay {
    forward: Relay,
    reverse: Relay,
}

impl DoubleRelay {
    pub fn new(port: i32) -> HalResult<Self> {
        let forward = Relay::new(port, true)?;
        let reverse = Relay::new(port, false)?;

        Ok(DoubleRelay { forward, reverse })
    }

    /// Does not validate that `forward` or `backward` are set to the right value
    pub fn from_relays(forward: Relay, reverse: Relay) -> Self {
        DoubleRelay { forward, reverse }
    }
    
    pub fn set_active(&self, fwd: bool, rev: bool) -> HalResult<()> {
        self.forward.set_active(fwd)?;
        self.reverse.set_active(rev)
    }

    pub fn set(&self, value: DualValue) -> HalResult<()> {
        Ok(match value {
            DualValue::On      => self.set_active(true, true)?,
            DualValue::Forward => self.set_active(true, false)?,
            DualValue::Reverse => self.set_active(false, true)?,
            DualValue::Off     => self.set_active(false, false)?,
        })
    }

    pub fn get(&self) -> HalResult<DualValue> {
        Ok(match (self.forward.get_active()?, self.reverse.get_active()?) {
            (true, true)   => DualValue::On,
            (true, false)  => DualValue::Forward,
            (false, true)  => DualValue::Backward,
            (false, false) => DualValue::Off,
        })
    }
}

// TODO: do we actually want all this?
/// Practically the equivalent the the wpilib Relay class in get/set behavior
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AnyRelay {
    Forward(Relay),
    Backward(Relay),
    Both(DoubleRelay),
}

impl AnyRelay {
    pub fn get(&self) -> HalResult<DualValue> {
        match self {
            &AnyRelay::Both(double) => double.get(),
            &AnyRelay::Forward(relay) | &AnyRelay::Reverse(relay) =>
                fwd.get().map(|active| if active { DualValue::On } else { DualValue::Off })
        }
    }

    /// Set the state of this relay.
    /// - `On` and `Off` values always work, regardless of relay type
    /// - `Forward` works on forward and dual relays
    /// - `Reverse` worls on reverse and dual relays
    /// Attempting to use Forward on a reverse relay or vice versa will cause a panic.
    pub fn set(&self, value: DualValue) -> HalResult<()> {
        match self {
            &AnyRelay::Both(double) => double.set(value),
            &AnyRelay::Forward(relay) => relay.set(match value {
                DualValue::On | DualValue::Forward => true,
                DualValue::Off => false,
                _ => panic!(),
            }),
            &AnyRelay::Backward(relay) => relay.set(match value {
                DualValue::On | DualValue::Backward => true,
                DualValue::Off => false,
                _ => panic!(),
            }),
        }
    }
}
