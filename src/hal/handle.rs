pub trait Handle {
    fn new(port: i32) -> Self;
    fn get_handle(&self) -> i32;
    fn transmute_handle<H: Handle>(&self) -> Box<H> {
        Box::new(H::new(self.get_handle()))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AnalogInputHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct GyroHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct PortHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct AnalogOutputHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct AnalogTriggerHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct CompressorHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct CounterHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct DigitalHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct DigitalPwmHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct RelayHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct SolenoidHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct InterruptHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct NotifierHandle(pub i32);
#[derive(Copy, Clone, Debug)]
pub struct SpiHandle(pub i32);

impl Handle for AnalogInputHandle {
    fn new(port: i32) -> AnalogInputHandle {
        AnalogInputHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for GyroHandle {
    fn new(port: i32) -> GyroHandle {
        GyroHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for PortHandle {
    fn new(port: i32) -> PortHandle {
        PortHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for AnalogOutputHandle {
    fn new(port: i32) -> AnalogOutputHandle {
        AnalogOutputHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for AnalogTriggerHandle {
    fn new(port: i32) -> AnalogTriggerHandle {
        AnalogTriggerHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for CompressorHandle {
    fn new(port: i32) -> CompressorHandle {
        CompressorHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for CounterHandle {
    fn new(port: i32) -> CounterHandle {
        CounterHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for DigitalHandle {
    fn new(port: i32) -> DigitalHandle {
        DigitalHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for DigitalPwmHandle {
    fn new(port: i32) -> DigitalPwmHandle {
        DigitalPwmHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for RelayHandle {
    fn new(port: i32) -> RelayHandle {
        RelayHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for SolenoidHandle {
    fn new(port: i32) -> SolenoidHandle {
        SolenoidHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for InterruptHandle {
    fn new(port: i32) -> InterruptHandle {
        InterruptHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for NotifierHandle {
    fn new(port: i32) -> NotifierHandle {
        NotifierHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
}

impl Handle for SpiHandle {
    fn new(port: i32) -> SpiHandle {
        SpiHandle(port)
    }
    fn get_handle(&self) -> i32 {
        self.0
    }
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
