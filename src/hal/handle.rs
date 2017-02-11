///
pub trait Handle {
    ///
    fn new(port: i32) -> Self;
    /// Get the
    fn get_handle(&self) -> i32;
    /// Transmutes this handle's type into `H`'s type
    fn transmute_handle<H: Handle>(&self) -> Box<H> {
        Box::new(H::new(self.get_handle()))
    }
}

macro_rules! gen_handles {
   ($($typ:ident),*) => (
       $(
           #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
           pub struct $typ(pub i32);
           impl Handle for $typ {
               fn new(port: i32) -> $typ { $typ(port) }
               fn get_handle(&self) -> i32 { self.0 }
           }
       )*
   );
}

gen_handles! {
    AnalogInputHandle,
    GyroHandle,
    PortHandle,
    AnalogOutputHandle,
    AnalogTriggerHandle,
    CompressorHandle,
    CounterHandle,
    DigitalHandle,
    DigitalPwmHandle,
    RelayHandle,
    SolenoidHandle,
    InterruptHandle,
    NotifierHandle,
    SpiHandle
}
