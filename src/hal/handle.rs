pub trait Handle {
    fn new(port: i32) -> Self;
    fn get_handle(&self) -> i32;
    fn transmute_handle<H>(&self) -> H where H: Self {
        H::new(self.get_handle())
    }
}

pub struct AnalogInputHandle(i32);
pub struct GyroHandle(i32);
pub struct PortHandle(i32);
pub struct AnalogOutputHandle(i32);
pub struct AnalogTriggerHandle(i32);
pub struct CompressorHandle(i32);
pub struct CounterHandle(i32);
pub struct DigitalHandle(i32);
pub struct DigitalPwmHandle(i32);
pub struct RelayHandle(i32);
pub struct SolenoidHandle(i32);
pub struct InterruptHandle(i32);
pub struct NotifierHandle(i32);

impl Handle for AnalogInputHandle   {
    fn new(port: i32) -> AnalogInputHandle { AnalogInputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for GyroHandle          {
    fn new(port: i32) -> GyroHandle { GyroHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for PortHandle          {
    fn new(port: i32) -> PortHandle { PortHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogOutputHandle  {
    fn new(port: i32) -> AnalogOutputHandle { AnalogOutputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogTriggerHandle {
    fn new(port: i32) -> AnalogTriggerHandle { AnalogTriggerHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for AnalogOutputHandle  {
    fn new(port: i32) -> AnalogOutputHandle { AnalogOutputHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for CompressorHandle    {
    fn new(port: i32) -> CompressorHandle { CompressorHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for CounterHandle       {
    fn new(port: i32) -> CounterHandle { CounterHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for DigitalHandle       {
    fn new(port: i32) -> DigitalHandle { DigitalHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for DigitalPwmHandle    {
    fn new(port: i32) -> DigitalPwmHandle { DigitalPwmHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for RelayHandle         {
    fn new(port: i32) -> RelayHandle { RelayHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for SolenoidHandle      {
    fn new(port: i32) -> SolenoidHandle { SolenoidHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for InterruptHandle     {
    fn new(port: i32) -> InterruptHandle { InterruptHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}

impl Handle for NotifierHandle      {
    fn new(port: i32) -> NotifierHandle { NotifierHandle(port) }
    fn get_handle(&self) -> i32 { self.0 }
}


// macro_rules! gen_handles {
//    ($($typ:ident),*) => (
//        $(
//            pub struct $typ(i32);
//            impl Handle for $typ {
//                fn get_handle(&self) -> i32 { self.0 }
//            }
//        )*
//    );
// }

// gen_handles! {
//    AnalogInputHandle
//    GyroHandle,
//    PortHandle,
//    AnalogOutputHandle,
//    AnalogTriggerHandle,
//    AnalogOutputHandle,
//    CompressorHandle,
//    CounterHandle,
//    DigitalHandle,
//    DigitalPwmHandle,
//    RelayHandle,
//    SolenoidHandle,
//    InterruptHandle
// }