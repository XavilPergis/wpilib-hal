use raw::*;
use hal::enums::*;
use hal::structs::*;
use hal::error::*;
use std::ffi::{CStr, CString};
use std::mem;
use hal::serial::SerialPort;

pub mod accelerometer {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    pub fn set_accelerometer_active(active: bool) {
        unsafe { HAL_SetAccelerometerActive(active as HAL_Bool) }
    }

    pub fn set_accelerometer_range<T>(range: T) where T: Into<AccelerometerRange> {
        unsafe { HAL_SetAccelerometerRange(range.into()) }
    }

    pub fn get_accelerometer_x() -> f64 {
        unsafe { HAL_GetAccelerometerX() }
    }

    pub fn get_accelerometer_y() -> f64 {
        unsafe { HAL_GetAccelerometerY() }
    }

    pub fn get_accelerometer_z() -> f64 {
        unsafe { HAL_GetAccelerometerZ() }
    }
}

pub mod accumulator {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    pub fn is_accumulator_channel(port: AnalogInputHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_IsAccumulatorChannel(port.get_handle()) ].map(|n| n != 0)
    }

    pub fn init_accumulator(port: AnalogInputHandle) -> HalResult<()> {
        hal_call![ ptr HAL_InitAccumulator(port.get_handle()) ]
    }

    pub fn reset_accumulator(port: AnalogInputHandle) -> HalResult<()> {
        hal_call![ ptr HAL_ResetAccumulator(port.get_handle()) ]
    }

    pub fn set_accumulator_center(port: AnalogInputHandle, center: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAccumulatorCenter(port.get_handle(), center) ]
    }

    pub fn set_accumulator_deadband(port: AnalogInputHandle, deadband: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAccumulatorDeadband(port.get_handle(), deadband) ]
    }

    pub fn get_accumulator_value(port: AnalogInputHandle) -> HalResult<i64> {
        hal_call![ ptr HAL_GetAccumulatorValue(port.get_handle()) ]
    }

    pub fn get_accumulator_count(port: AnalogInputHandle) -> HalResult<i64> {
        hal_call![ ptr HAL_GetAccumulatorCount(port.get_handle()) ]
    }
}
// TODO: Do we really need this?
//pub fn HAL_GetAccumulatorOutput(analogPortHandle: HAL_AnalogInputHandle,
//                                value: *mut i64,
//                                count: *mut i64,
//                                status: *mut i32);

pub mod analog_gyro {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;
    use super::analog_input::AnalogInput;

    pub struct AnalogGyro {
        port: GyroHandle
    }

    impl AnalogGyro {
        pub fn new(port: AnalogInputHandle) -> HalResult<AnalogGyro> {
            self::initialize_analog_gyro(port).map(|gh| AnalogGyro {
                port: port
            })
        }
    }

    impl AnalogInput for AnalogGyro {
        fn get_handle(&self) -> AnalogInputHandle {
            self.port.transmute_handle::<AnalogInputHandle>()
        }
    }

    pub fn initialize_analog_gyro(port: AnalogInputHandle) -> HalResult<GyroHandle> {
        hal_call![ ptr HAL_InitializeAnalogGyro(port.get_handle()) ].map(From::from)
    }

    pub fn setup_analog_gyro(port: GyroHandle) -> HalResult<()> {
        hal_call![ ptr HAL_SetupAnalogGyro(port.get_handle()) ]
    }

    pub fn free_analog_gyro(port: GyroHandle) {
        unsafe { HAL_FreeAnalogGyro(port.get_handle()) }
    }

    pub fn set_analog_gyro_parameters(handle: GyroHandle, volts_per_degree_per_second: f64, offset: f64, center: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogGyroParameters(handle.get_handle(), volts_per_degree_per_second, offset, center) ]
    }

    pub fn set_analog_gyro_volts_per_degree_per_second(handle: GyroHandle, vds: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogGyroVoltsPerDegreePerSecond(handle.get_handle(), vds) ]
    }

    pub fn reset_analog_gyro(handle: GyroHandle) -> HalResult<()> {
        hal_call![ ptr HAL_ResetAnalogGyro(handle.get_handle()) ]
    }

    pub fn calibrate_analog_gyro(handle: GyroHandle) -> HalResult<()> {
        hal_call![ ptr HAL_CalibrateAnalogGyro(handle.get_handle()) ]
    }

    pub fn set_analog_gyro_deadband(handle: GyroHandle, volts: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogGyroDeadband(handle.get_handle(), volts) ]
    }

    pub fn get_analog_gyro_angle(handle: GyroHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogGyroAngle(handle.get_handle()) ]
    }

    pub fn get_analog_gyro_rate(handle: GyroHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogGyroRate(handle.get_handle()) ]
    }

    pub fn get_analog_gyro_offset(handle: GyroHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogGyroOffset(handle.get_handle()) ]
    }

    pub fn get_analog_gyro_center(handle: GyroHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogGyroCenter(handle.get_handle()) ]
    }
}

pub mod analog_input {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    // TODO: rait or struct?
    pub trait AnalogInput {
        fn get_handle(&self) -> AnalogInputHandle;

        fn get_raw(&self) -> HalResult<i32> {
            self::get_analog_value(self.get_handle())
        }

        fn get_voltage(&self) -> HalResult<f64> {
            self::get_analog_voltage(self.get_handle())
        }

        fn get_average_raw(&self) -> HalResult<i32> {
            self::get_analog_average_value(self.get_handle())
        }

        fn get_average_voltage(&self) -> HalResult<f64> {
            self::get_analog_average_voltage(self.get_handle())
        }
    }

    pub struct IrSensor {
        handle: AnalogInputHandle
    }

    impl AnalogInput for IrSensor {
        fn get_handle(&self) -> AnalogInputHandle {
            self.handle
        }
    }

    pub fn initialize_analog_input_port(handle: PortHandle) -> HalResult<AnalogInputHandle> {
        hal_call![ ptr HAL_InitializeAnalogInputPort(handle.get_handle()) ].map(|n| AnalogInputHandle(n))
    }

    pub fn free_analog_input_port(handle: AnalogInputHandle) {
        unsafe { HAL_FreeAnalogInputPort(handle.get_handle()) }
    }

    // What the fuck is this
    pub fn check_analog_module(module: i32) -> bool {
        unsafe { HAL_CheckAnalogModule(module) != 0 }
    }

    // Also what the fuck
    pub fn check_analog_input_channel(channel: i32) -> bool {
        unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
    }

    pub fn set_analog_sample_rate(samples_per_second: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogSampleRate(samples_per_second) ]
    }

    pub fn get_analog_sample_rate() -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogSampleRate() ]
    }

    pub fn set_analog_average_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogAverageBits(handle.get_handle(), bits) ]
    }

    pub fn get_analog_average_bits(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogAverageBits(handle.get_handle()) ]
    }

    pub fn set_analog_oversample_bits(handle: AnalogInputHandle, bits: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogOversampleBits(handle.get_handle(), bits) ]
    }

    pub fn get_analog_oversample_bits(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogOversampleBits(handle.get_handle()) ]
    }

    pub fn get_analog_value(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogValue(handle.get_handle()) ]
    }

    pub fn get_analog_average_value(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogAverageValue(handle.get_handle()) ]
    }

    pub fn get_analog_volts_to_value(handle: AnalogInputHandle, voltage: f64) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogVoltsToValue(handle.get_handle(), voltage) ]
    }

    pub fn get_analog_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogVoltage(handle.get_handle()) ]
    }

    pub fn get_analog_average_voltage(handle: AnalogInputHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogAverageVoltage(handle.get_handle()) ]
    }

    pub fn get_analog_lsb_weight(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogLSBWeight(handle.get_handle()) ]
    }

    pub fn get_analog_offset(handle: AnalogInputHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAnalogOffset(handle.get_handle()) ]
    }
    
}

pub mod analog_output {

    pub trait AnalogOutput {
        fn get(&self) -> HalResult<f64>;
        fn set(&mut self) -> HalResult<()>;
    }

    pub fn initialize_analog_output_port(handle: PortHandle) -> HalResult<AnalogOutputHandle> {
        hal_call![ ptr HAL_InitializeAnalogOutputPort(handle.get_handle()) ].map(|n| AnalogOutputHandle(n))
    }

    pub fn free_analog_output_port(handle: AnalogOutputHandle) {
        unsafe { HAL_FreeAnalogOutputPort(handle.get_handle()) }
    }

    pub fn set_analog_output(handle: AnalogOutputHandle, voltage: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogOutput(handle.get_handle(), voltage) ]
    }

    pub fn get_analog_output(handle: AnalogOutputHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetAnalogOutput(handle.get_handle()) ]
    }

    pub fn check_analog_output_channel(channel: i32) -> bool {
        unsafe { HAL_CheckAnalogOutputChannel(channel) != 01 }
    }
}

pub mod analog_trigger {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    //pub fn initialize_analog_trigger(handle: AnalogInputHandle)
    // FIXME
    //    pub fn HAL_InitializeAnalogTrigger(portHandle: HAL_AnalogInputHandle,
    //                                       index: *mut i32,
    //                                       status: *mut i32)
    //                                       -> HAL_AnalogTriggerHandle;

    pub fn clean_analog_trigger(handle: AnalogTriggerHandle) -> HalResult<()> {
        hal_call![ ptr HAL_CleanAnalogTrigger(handle.get_handle()) ]
    }

    pub fn set_analog_trigger_limits_raw(handle: AnalogTriggerHandle, lower: i32, upper: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogTriggerLimitsRaw(handle.get_handle(), lower, upper) ]
    }

    pub fn set_analog_trigger_limits_voltage(handle: AnalogTriggerHandle, lower: f64, upper: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogTriggerLimitsVoltage(handle.get_handle(), lower, upper) ]
    }

    pub fn set_analog_trigger_avergaed(handle: AnalogTriggerHandle, use_averaged_value: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogTriggerAveraged(handle.get_handle(), use_averaged_value as HAL_Bool) ]
    }

    pub fn set_analog_trigger_filtered(handle: AnalogTriggerHandle, use_filtered_value: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetAnalogTriggerFiltered(handle.get_handle(), use_filtered_value as HAL_Bool) ]
    }

    pub fn get_analog_trigger_in_window(handle: AnalogTriggerHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetAnalogTriggerInWindow(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_analog_trigger_state(handle: AnalogTriggerHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetAnalogTriggerTriggerState(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_analog_trigger_output(handle: AnalogTriggerHandle, trigger_type: AnalogTriggerType) -> HalResult<bool> {
        hal_call![ ptr HAL_GetAnalogTriggerOutput(handle.get_handle(), trigger_type.into()) ].map(|n| n != 0)
    }
}

// FIXME: For some reason, we're completely missing CAN

pub mod can {

    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    pub struct CANStreamMessage {
        pub message_id: u32,
        pub time_stamp: u32,
        pub data: [u8; 8usize],
        pub data_size: u8,
    }

    pub struct CANDevice {

    }

    impl CANDevice {

    }

    pub trait CANTransmittable {
        fn tx_pack(&self, arb_id: u32, offset: u8);
        fn tx_unpack(arb_id: u32, offset: u8) -> Self;
        fn rx_unpack(arb_id: u32, offset: u8) -> Self;
    }

    impl CANTransmittable for u8 {
        fn tx_pack(&self, arb_id: u32, offset: u8) {
            can_tx_pack_int8(arb_id, offset, *self)
        }

        fn tx_unpack(arb_id: u32, offset: u8) -> u8 {
            can_tx_unpack_int8(arb_id, offset)
        }

        fn rx_unpack(arb_id: u32, offset: u8) -> u8 {
            can_rx_unpack_int8(arb_id, offset)
        }
    }

    impl CANTransmittable for u16 {
        fn tx_pack(&self, arb_id: u32, offset: u16) {
            can_tx_pack_int16(arb_id, offset, value)
        }

        fn tx_unpack(arb_id: u32, offset: u8) -> u16 {
            can_tx_unpack_int16(arb_id, offset)
        }

        fn rx_unpack(arb_id: u32, offset: u8) -> u16 {
            can_rx_unpack_int16(arb_id, offset)
        }
    }

    impl CANTransmittable for u32 {
        fn tx_pack(&self, arb_id: u32, offset: u32) {
            can_tx_pack_int32(arb_id, offset, value)
        }

        fn tx_unpack(arb_id: u32, offset: u8) -> u32 {
            can_tx_unpack_int32(arb_id, offset)
        }

        fn rx_unpack(arb_id: u32, offset: u8) -> u32 {
            can_rx_unpack_int32(arb_id, offset)
        }
    }

    // FIXME
    impl CANTransmittable for f64 {
        fn tx_pack(&self, arb_id: u32, offset: f64) {
            can_tx_pack_int32(arb_id, offset, value)
        }

        fn tx_unpack(arb_id: u32, offset: u8) -> f64 {
            can_tx_unpack_int32(arb_id, offset)
        }

        fn rx_unpack(arb_id: u32, offset: u8) -> f64 {
            can_rx_unpack_int32(arb_id, offset)
        }
    }

    // pub fn FRC_NetworkCommunication_CANSessionMux_sendMessage(messageID: u32, data: *const u8, dataSize: u8, periodMs: i32) -> HalResult<()> {}
    // pub fn FRC_NetworkCommunication_CANSessionMux_receiveMessage(messageID: *mut u32, messageIDMask: u32, data: *mut u8, dataSize: *mut u8, timeStamp: *mut u32) -> HalResult<()> {}
    // pub fn FRC_NetworkCommunication_CANSessionMux_openStreamSession(sessionHandle: *mut u32, messageID: u32, messageIDMask: u32, maxMessages: u32) -> HalResult<()>;
    // pub fn FRC_NetworkCommunication_CANSessionMux_closeStreamSession(sessionHandle: u32);
    // pub fn FRC_NetworkCommunication_CANSessionMux_readStreamSession(sessionHandle: u32, messages: *mut tCANStreamMessage, messagesToRead: u32, messagesRead: *mut u32) -> HalResult<()>;
    // pub fn FRC_NetworkCommunication_CANSessionMux_getCANStatus(percentBusUtilization: *mut f32, busOffCount: *mut u32, txFullCount: *mut u32, receiveErrorCount: *mut u32, transmitErrorCount: *mut u32) -> HalResult<()>;
 
    pub fn can_tx_pack<C>(arb_id: u32, offset: u8, value: C) where C: CANTransmittable {
        value.tx_pack(arb_id, offset, value)
    }

    pub fn can_tx_unpack<C>(arb_id: u32, offset: u8) -> C where C: CANTransmittable {
        C::tx_unpack(arb_id, offset)
    }

    pub fn can_tx_send(arb_id: u32, length: u8, period: i32) {
        unsafe { canTxSend(arb_id) }
    }

    pub fn can_tx_pack_int8(arb_id: u32, offset: u8, value: u8) {
        unsafe  { canTxPackInt8(arb_id, offset, value) }
    }

    pub fn can_tx_pack_int16(arb_id: u32, offset: u8, value: u16) {
        unsafe { canTxPackInt16(arb_id, offset, value) }
    }

    pub fn can_tx_pack_int32(arb_id: u32, offset: u8, value: u32) {
        unsafe { canTxPackInt32(arb_id, offset, value) }
    }

    pub fn can_tx_pack_fxp16(arb_id: u32, offset: u8, value: f64) {
        unsafe { canTxPackFXP16(arb_id, offset, value) }
    }

    pub fn can_tx_pack_fxp32(arb_id: u32, offset: u8, value: f64) {
        unsafe { canTxPackFXP32(arb_id, offset, value) }
    }

    pub fn can_tx_unpack_int8(arb_id: u32, offset: u8) -> u8 {
        unsafe { canTxUnpackInt8(arb_id, offset) }
    }

    pub fn can_tx_unpack_int32(arb_id: u32, offset: u8) -> u32 {
        unsafe { canTxUnpackInt32(arb_id, offset) }
    }

    pub fn can_tx_unpack_int16(arb_id: u32, offset: u8) -> u16 {
        unsafe { canTxUnpackInt16(arb_id, offset) }
    }

    pub fn can_tx_unpack_fxp16(arb_id: u32, offset: u8) -> f64 {
        unsafe { canTxUnpackFXP16(arb_id, offset) }
    }

    pub fn can_tx_unpack_fxp32(arb_id: u32, offset: u8) -> f64 {
        unsafe { canTxUnpackFXP32(arb_id, offset) }
    }

    pub fn can_rx_receive(arb_id: u32) -> bool {
        unsafe { canRxReceive(arb_id) }
    }

    pub fn can_rx_unpack_int8(arb_id: u32, offset: u8) -> u8 {
        unsafe { canRxUnpackInt8(arb_id, offset) }
    }

    pub fn can_rx_unpack_int16(arb_id: u32, offset: u8) -> u16 {
        unsafe { canRxUnpackInt16(arb_id, offset) }
    }

    pub fn can_rx_unpack_int32(arb_id: u32, offset: u8) -> u32 {
        unsafe { canRxUnpackInt32(arb_id, offset) }
    }

    pub fn can_rx_unpack_fxp16(arb_id: u32, offset: u8) -> f64 {
        unsafe { canRxUnpackFXP16(arb_id, offset) }
    }

    pub fn can_rx_unpack_fxp32(arb_id: u32, offset: u8) -> f64 {
        unsafe { canRxUnpackFXP32(arb_id, offset) }
    }

}

pub mod compressor {
    use raw::*;
    use hal::enums::*;
    use hal::structs::*;
    use hal::error::*;

    pub fn initialize_compressor(module: i32) -> HalResult<CompressorHandle> {
        hal_call![ ptr HAL_InitializeCompressor(module) ]
    }

    pub fn check_compressor_module(module: i32) -> bool {
        unsafe { HAL_CheckCompressorModule(module) != 0 }
    }

    pub fn get_compressor(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressor(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn set_compressor_closed_loop_control(handle: CompressorHandle, value: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCompressorClosedLoopControl(handle.get_handle(), value as HAL_Bool) ]
    }

    pub fn get_compressor_closed_loop_control(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorClosedLoopControl(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_pressure_switch(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorPressureSwitch(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_current(handle: CompressorHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetCompressorCurrent(handle.get_handle()) ]
    }

    pub fn get_compressor_current_too_high_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorCurrentTooHighFault(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_current_too_high_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorCurrentTooHighStickyFault(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_shorted_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorShortedStickyFault(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_shorted_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorShortedFault(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_not_connected_sticky_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorNotConnectedStickyFault(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_compressor_not_connected_fault(handle: CompressorHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetCompressorNotConnectedFault(handle.get_handle()) ].map(|n| n != 0)
    }
}


pub fn get_system_clock_ticks_per_microsecond() -> i32 {
    unsafe { HAL_GetSystemClockTicksPerMicrosecond() }
}

pub mod counter {
    // FIXME
    pub fn initialize_counter(mode: CounterMode, index: *mut i32) -> HalResult<HAL_CounterHandle> {
        hal_call![ ptr HAL_InitializeCounter(mode.into()) ]
    }

    pub fn free_counter(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_FreeCounter(handle.get_handle()) ]
    }

    pub fn set_counter_average_size(handle: CounterHandle, size: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterAverageSize(handle.get_handle(), size) ]
    }

    pub fn set_counter_up_source<H>(handle: CounterHandle, digital_source_handle: &H, trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
        hal_call![ ptr HAL_SetCounterUpSource(handle.get_handle(), digital_source_handle.get_handle(), trigger_type.into()) ]
    }

    pub fn set_counter_up_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterUpSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
    }

    pub fn clear_counter_up_source(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_ClearCounterUpSource(handle.get_handle()) ]
    }

    pub fn set_counter_down_source<H>(handle: CounterHandle, digital_source_handle: &H, analog_trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
        hal_call![ ptr HAL_SetCounterDownSource(handle.get_handle(), digital_source_handle.get_handle(), analog_trigger_type.into()) ]
    }

    pub fn set_counter_down_source_edge(handle: CounterHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterDownSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
    }

    pub fn clear_counter_down_source(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_ClearCounterDownSource(handle.get_handle()) ]
    }

    pub fn set_counter_up_down_mode(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterUpDownMode(handle.get_handle()) ]
    }

    pub fn set_counter_external_direction_mode(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterExternalDirectionMode(handle.get_handle()) ]
    }

    pub fn set_counter_semi_period_mode(handle: CounterHandle, high_semi_period: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterSemiPeriodMode(handle.get_handle(), high_semi_period as HAL_Bool) ]
    }

    pub fn set_counter_pulse_length_mode(handle: CounterHandle, threshold: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterPulseLengthMode(handle.get_handle()) ]
    }

    pub fn get_counter_samples_to_average(handle: CounterHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetCounterSamplesToAverage(handle.get_handle()) ]
    }

    pub fn set_counter_samples_to_average(handle: CounterHandle, samples_to_average: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterSamplesToAverage(handle.get_handle(), samples_to_average) ]
    }

    pub fn reset_counter(handle: CounterHandle) -> HalResult<()> {
        hal_call![ ptr HAL_ResetCounter(handle.get_handle()) ]
    }

    pub fn get_counter(handle: CounterHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetCounter(handle.get_handle()) ]
    }

    pub fn get_counter_period(handle: CounterHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetCounterPeriod(handle.get_handle()) ]
    }

    pub fn set_counter_max_period(handle: CounterHandle, max_period: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterMaxPeriod(handle.get_handle(), max_period) ]
    }

    pub fn set_counter_update_when_empty(handle: CounterHandle, enabled: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterUpdateWhenEmpty(handle.get_handle(), enabled as HAL_Bool) ]
    }

    pub fn get_counter_stopped(handle: CounterHandle) -> HalResult<HAL_Bool> {
        hal_call![ ptr HAL_GetCounterStopped(handle.get_handle()) ]
    }

    pub fn get_counter_direction(handle: CounterHandle) -> HalResult<HAL_Bool> {
        hal_call![ ptr HAL_GetCounterDirection(handle.get_handle()) ]
    }

    pub fn set_counter_reverse_direction(handle: CounterHandle, reverse_direction: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetCounterReverseDirection(handle.get_handle(), reverse_direction as HAL_Bool) ]
    }
}

pub mod dio {
    pub fn initialize_dio_port(handle: PortHandle, input: bool) -> HalResult<DigitalHandle> {
        hal_call![ ptr HAL_InitializeDIOPort(handle.get_handle(), input as HAL_Bool) ].map(From::from)
    }

    pub fn check_dio_channel(channel: i32) -> bool {
        unsafe { HAL_CheckDIOChannel(channel) != 0 }
    }

    pub fn free_dio_port(dio_port_handle: DigitalHandle) {
        unsafe { HAL_FreeDIOPort(dio_port_handle.get_handle()) }
    }

    pub fn allocate_digital_pwm() -> HalResult<DigitalPwmHandle> {
        hal_call![ ptr HAL_AllocateDigitalPWM() ].map(|n| DigitalPwmHandle(n))
    }

    pub fn free_digital_pwm(pwm_generator: DigitalPwmHandle) -> HalResult<()> {
        hal_call![ ptr HAL_FreeDigitalPWM(pwm_generator.get_handle()) ]
    }

    pub fn set_digital_pwm_rate(rate: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetDigitalPWMRate(rate) ]
    }

    pub fn set_digital_pwm_duty_cycle(pwm_generator: DigitalPwmHandle, duty_cycle: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetDigitalPWMDutyCycle(pwm_generator.get_handle(), duty_cycle) ]
    }

    pub fn set_digital_pwm_output_channel(pwm_generator: DigitalPwmHandle, channel: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetDigitalPWMOutputChannel(pwm_generator.get_handle(), channel) ]
    }

    pub fn set_dio(handle: DigitalHandle, value: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetDIO(handle.get_handle(), value as HAL_Bool) ]
    }

    pub fn get_dio(handle: DigitalHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetDIO(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_dio_direction(handle: DigitalHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetDIODirection(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn pulse(handle: DigitalHandle, pulse_length: f64) -> HalResult<()> {
        hal_call![ ptr HAL_Pulse(handle.get_handle(), pulse_length) ]
    }

    pub fn is_pulsing(handle: DigitalHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_IsPulsing(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn is_any_pulsing() -> HalResult<bool> {
        hal_call![ ptr HAL_IsAnyPulsing() ].map(|n| n != 0)
    }

    pub fn set_filter_select(handle: DigitalHandle, filter_index: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetFilterSelect(handle.get_handle(), filter_index) ]
    }

    pub fn get_filter_select(handle: DigitalHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetFilterSelect(handle.get_handle()) ]
    }

    pub fn set_filter_period(filter_index: i32, value: i64) -> HalResult<()> {
        hal_call![ ptr HAL_SetFilterPeriod(filter_index, value) ]
    }

    pub fn get_filter_period(filter_index: i32) -> HalResult<i64> {
        hal_call![ ptr HAL_GetFilterPeriod(filter_index) ]
    }
}

pub mod driverstation {
    //! This module ports WPILIB's `HAL/DriverStation.h` to Rust

    use raw::*;
    use hal::error::*;

    use std::ffi::CString;
    use std::mem;

    use time::Duration;

    /// The maximum amount of axes that a controller can have. Realistically, this is 3 or 4.
    pub const MAX_JOYSTICK_AXES: usize = 12;
    /// The maximum amount of POVs that a controller can have. POVs are the little D-pad like things
    /// on the top of the joystick.
    pub const MAX_JOYSTICK_POVS: usize = 12;

    /// Where the driver station is on the field. Either `Red` or `Blue` with a position from 1 to 3
    /// or an invalid station.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum AllianceStation {
        /// Red alliance station
        Red(u8),
        /// Blue alliance station
        Blue(u8),
        /// Invalid station
        Invalid
    }

    // TODO: More insightful comments
    /// What modes the driver station is in. Only one of `enabled`, `autonomous`, `test`, and `stopped`
    /// should be on at a time.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ControlWord {
        /// Whether the driver station is enabled
        pub enabled: bool,
        /// Whether the driver station is in autonomous mode
        pub autonomous: bool,
        /// Whether the driver station is in test mode
        pub test: bool,
        /// Whether the driver station is stopped
        pub stopped: bool,
        /// Whether the Field Managment System is attached
        pub fms_attached: bool,
        /// Whether the Driver Station is attached
        pub ds_attached: bool
    }

    /// TODO: Figure out what a joystick type actually is
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum JoystickType {
        /// An unknown type of joystick. TODO: What is this?
        Unknown(i32)
    }

    /// What mode the user program is running in.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum UserProgramMode {
        /// The user program is starting
        Starting,
        /// The user program is disabled
        Disabled,
        /// The user program is in autonomous mode
        Autonomous,
        /// The user proram is in tele-operated mode
        TeleOperated,
        /// The user program is in test mode
        Test
    }

    // TODO: What is this?
    pub fn set_error_data(errors: &str, errors_length: i32, wait_ms: i32) -> HalResult<()> {
        hal_call![ ret HAL_SetErrorData(CString::new(errors).map_err(HalError::from)?.as_ptr(), errors_length, wait_ms) ]
    }

    /// Gets a joystick's descriptor from the driver station
    pub fn get_joystick_descriptor(joystick_num: i32) -> HalResult<JoystickDescriptor> {
        let mut descriptor = unsafe { mem::zeroed() };
        hal_call!(ret HAL_GetJoystickDescriptor(joystick_num, &mut descriptor as *mut HAL_JoystickDescriptor))?;

        Ok(JoystickDescriptor::from(descriptor))
    }

    /// Gets the rotations on each "axis" of a joystick. An axis is basically just something that can
    /// be somewhere in a range of values.
    pub fn get_joystick_axes(joystick_num: i32) -> HalResult<JoystickAxes> {
        let mut raw_axes = unsafe { mem::zeroed() };
        hal_call!(ret HAL_GetJoystickAxes(joystick_num, &mut raw_axes as *mut HAL_JoystickAxes))?;

        Ok(JoystickAxes::from(raw_axes))
    }

    /// Gets the state of all the POVs on the joystick.
    pub fn get_joystick_povs(joystick_num: i32) -> HalResult<JoystickPovs> {
        let mut raw_povs = unsafe { mem::zeroed() };
        hal_call!(ret HAL_GetJoystickPOVs(joystick_num, &mut raw_povs as *mut HAL_JoystickPOVs))?;

        Ok(JoystickPovs::from(raw_povs))
    }

    /// Gets what buttons are pressed on a joystick
    pub fn get_joystick_buttons(joystick_num: i32) -> HalResult<JoystickButtons> {
        let mut raw_buttons: HAL_JoystickButtons = unsafe { mem::zeroed() };
        hal_call!(ret HAL_GetJoystickButtons(joystick_num, &mut raw_buttons as *mut HAL_JoystickButtons))?;

        Ok(JoystickButtons::from(raw_buttons))
    }

    /// Gets whether the joystick is an xbox controller or not
    pub fn get_joystick_is_xbox(joystick_num: i32) -> HalResult<bool> {
        Ok(get_joystick_descriptor(joystick_num)?.is_xbox)
    }

    /// TODO: Figure out what a joystick type is
    pub fn get_joystick_type(joystick_num: i32) -> HalResult<JoystickType> {
        Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.stick_type))
    }

    /// Gets the name of a joystick. This will return a string with a length no greater than 256.
    pub fn get_joystick_name(joystick_num: i32) -> HalResult<String> {
        Ok(get_joystick_descriptor(joystick_num)?.name)
    }

    // TODO: Figure out what a joystick type is
    pub fn get_joystick_axis_type(joystick_num: i32, axis: i32) -> HalResult<JoystickType> {
        if axis >= 0 {
            Ok(JoystickType::from(get_joystick_descriptor(joystick_num)?.axis_types[axis as usize] as i32))
        } else {
            Err(HalError::Hal(FfiError::ParameterOutOfRange))
        }
    }

    // TODO: What is this?
    pub fn set_joystick_outputs(joystick_num: i32, outputs: i64, left_rumble: i32, right_rumble: i32) -> HalResult<()> {
        hal_call!(ret HAL_SetJoystickOutputs(joystick_num, outputs, left_rumble, right_rumble))
    }

    // TODO: What are we actually observing? This should be called in the main DS loop
    pub fn observe_user_program(mode: UserProgramMode) {
        match mode {
            UserProgramMode::Starting => self::observe_user_program_starting(),
            UserProgramMode::Disabled => self::observe_user_program_disabled(),
            UserProgramMode::Autonomous => self::observe_user_program_autonomous(),
            UserProgramMode::TeleOperated => self::observe_user_program_teleop(),
            UserProgramMode::Test => self::observe_user_program_test()
        }
    }

    /// Initialize the driver station. Should only be called once.
    pub fn initialize_driver_station() {
        unsafe { HAL_InitializeDriverStation() };
    }

    /// Gets a control word directly from the driver station. The result should be cached for ~50ms
    pub fn get_control_word() -> HalResult<ControlWord> {
        let mut control_word: HAL_ControlWord = unsafe { mem::zeroed() };
        hal_call!(ret HAL_GetControlWord(&mut control_word as *mut HAL_ControlWord))?;

        Ok(ControlWord::from(control_word))
    }

    /// Blocks until the DS returns some data. Good for building concurrent abstractions.
    ///
    /// ## Example
    /// ```
    /// // create a channel for doing concurrent locking
    /// let (tx, rx) = mpsc::channel::<()>();
    /// thread::spawn(|| {
    ///     let thread_tx = tx.clone();
    ///     loop {
    ///         // Wait for the DS data to update
    ///         wait_for_ds_data();
    ///         // And then send an "unlock" to the receiver
    ///         thread_tx.send(()).unwrap();
    ///     }
    /// });
    /// ```
    pub fn wait_for_ds_data() {
        unsafe { HAL_WaitForDSData() };
    }

    /// Reports an error to the driver station
    pub fn send_error(is_error: bool, error_code: i32, is_lv_code: bool, details: &str, location: &str, call_stack: &str, print_message: bool) -> Result<(), HalError> {
        // CString::new() will return an `Err(NulError)` if there is a `\0` in the string passed in
        // Since this is a struct type error, it means that only `Err(NulError)` should ever be passed
        // in so we can safely transmute `NulError` into `HalError::NullError`
        let details_raw = CString::new(details).map_err(HalError::from)?;
        let location_raw = CString::new(location).map_err(HalError::from)?;
        let call_stack_raw = CString::new(call_stack).map_err(HalError::from)?;

        // TODO: Will the pointers be dropped here? I don't *think* so?
        hal_call!(ret HAL_SendError(is_error as HAL_Bool, error_code, is_lv_code as HAL_Bool,
        details_raw.as_ptr(), location_raw.as_ptr(), call_stack_raw.as_ptr(),
        print_message as HAL_Bool))
    }

    /// Gets where the driver station thinks it is.
    pub fn get_alliance_station() -> HalResult<AllianceStation> {
        let station_id = hal_call!(ptr HAL_GetAllianceStation())?;

        use ::raw::HAL_AllianceStationID;

        Ok(match station_id {
            HAL_AllianceStationID::HAL_AllianceStationID_kRed1 => AllianceStation::Red(1),
            HAL_AllianceStationID::HAL_AllianceStationID_kRed2 => AllianceStation::Red(2),
            HAL_AllianceStationID::HAL_AllianceStationID_kRed3 => AllianceStation::Red(3),
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue1 => AllianceStation::Blue(1),
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue2 => AllianceStation::Blue(2),
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue3 => AllianceStation::Blue(3)
        })
    }

    /// Gets the match time so far. This is not the *actual* match time, just an approximation of it.
    /// Since this is not the canonical match time, it cannot be used to dispute times or garuntee
    /// that a task completes before the match runs out.
    pub fn get_match_time_approx() -> HalResult<Duration> {
        let time = hal_call![ ptr HAL_GetMatchTime() ]?;

        // TODO: What the hell are the units that are returned!? Probably seconds...
        let time_ns = (time * 1_000_000_000f64) as i64;
        Ok(Duration::nanoseconds(time_ns))
    }

    pub fn observe_user_program_starting() {
        unsafe { HAL_ObserveUserProgramStarting() }
    }

    pub fn observe_user_program_disabled() {
        unsafe { HAL_ObserveUserProgramDisabled() }
    }

    pub fn observe_user_program_autonomous() {
        unsafe { HAL_ObserveUserProgramAutonomous() }
    }

    pub fn observe_user_program_teleop() {
        unsafe { HAL_ObserveUserProgramTeleop() }
    }

    pub fn observe_user_program_test() {
        unsafe { HAL_ObserveUserProgramTest() }
    }
}

/// Raw bindings to I2C functions
pub mod i2c {
    use raw::*;

    pub fn initialize_i2c(port: i32) -> HalResult<()> {
        hal_call![ ptr HAL_InitializeI2C(port) ]
    }

    // TODO
    //pub fn transaction_i2c(port: i32, device_address: i32, dataToSend: *mut u8, sendSize: i32, dataReceived: *mut u8, receiveSize: i32) -> i32 {
    //
    //}

    pub fn write_i2c(port: i32, device_address: i32, data_to_send: &[u8], send_size: i32) -> i32 {
        unsafe { HAL_WriteI2C(port, device_address, data_to_send.as_ptr() as *mut u8, send_size) }
    }

    pub fn read_i2c(port: i32, device_address: i32, buffer: &mut [u8], count: i32) -> i32 {
        unsafe { HAL_ReadI2C(port, device_address, buffer.as_mut_ptr() as *mut u8, count) }
    }

    pub fn close_i2c(port: i32) {
        unsafe { HAL_CloseI2C(port) }
    }
}

/// One whole big TODO
pub mod interrupt {
    use hal::raw::*;

    pub fn initialize_interrupts(watcher: bool) -> HalResult<InterruptHandle> {
        hal_call![ ptr HAL_InitializeInterrupts(watcher as HAL_Bool) ]
    }

    pub fn clean_interrupts(handle: InterruptHandle) -> HalResult<()> {
        hal_call![ ptr HAL_CleanInterrupts(handle.get_handle()) ]
    }

    pub fn wait_for_interrupt(handle: InterruptHandle, timeout: f64, ignore_previous: bool) -> HalResult<i64> {
        hal_call![ ptr HAL_WaitForInterrupt(handle.get_handle(), timeout, ignore_previous as HAL_Bool) ]
    }

    pub fn enable_interrupts(handle: InterruptHandle) -> HalResult<()> {
        hal_call![ ptr HAL_EnableInterrupts(handle.get_handle()) ]
    }

    pub fn disable_interrupts(handle: InterruptHandle) -> HalResult<()> {
        hal_call![ ptr HAL_DisableInterrupts(handle.get_handle()) ]
    }

    pub fn read_interrupt_rising_timestamp(handle: InterruptHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_ReadInterruptRisingTimestamp(handle.get_handle()) ]
    }

    pub fn read_interrupt_falling_timestamp(handle: InterruptHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_ReadInterruptFallingTimestamp(handle.get_handle()) ]
    }

    pub fn request_interrupts<H>(handle: InterruptHandle, digital_source_handle: &H, analog_trigger_type: AnalogTriggerType) -> HalResult<()> where H: Handle {
        hal_call![ ptr HAL_RequestInterrupts(handle.get_handle(), digital_source_handle.get_handle(), analog_trigger_type.into()) ]
    }

    // ?TODO
    pub fn attach_interrupt_handler(handle: InterruptHandle, handler: HAL_InterruptHandlerFunction, param: *mut ::std::os::raw::c_void) -> HalResult<()> {
        hal_call![ ptr HAL_AttachInterruptHandler(handle.get_handle()) ]
    }

    // ?TODO
    pub fn attach_interrupt_handler_threaded(handle: InterruptHandle, handler: HAL_InterruptHandlerFunction, param: *mut ::std::os::raw::c_void) -> HalResult<()> {
        hal_call![ ptr HAL_AttachInterruptHandlerThreaded(handle.get_handle()) ]
    }

    pub fn set_interrupt_up_source_edge(handle: InterruptHandle, rising_edge: bool, falling_edge: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetInterruptUpSourceEdge(handle.get_handle(), rising_edge as HAL_Bool, falling_edge as HAL_Bool) ]
    }
}

pub mod notifier {
    use raw::*;

    pub trait InterruptHandlerFunction {

    }

    pub fn initialize_notifier(process: HAL_NotifierProcessFunction, param: *mut ::std::os::raw::c_void) -> HalResult<NotifierHandle> {
        hal_call![ ptr HAL_InitializeNotifier() ]
    }

    pub fn initialize_notifier_threaded(process: HAL_NotifierProcessFunction, param: *mut ::std::os::raw::c_void) -> HalResult<NotifierHandle> {
        hal_call![ ptr HAL_InitializeNotifierThreaded() ]
    }

    pub fn clean_notifier(handle: NotifierHandle) -> HalResult<()> {
        hal_call![ ptr HAL_CleanNotifier(handle.get_handle()) ]
    }

    // Oh fuck
    pub fn get_notifier_param(handle: NotifierHandle) -> HalResult<*mut ::std::os::raw::c_void> {
        hal_call![ ptr HAL_GetNotifierParam(handle.get_handle()) ]
    }

    pub fn update_notifier_alarm(handle: NotifierHandle, trigger_time: u64) -> HalResult<()> {
        hal_call![ ptr HAL_UpdateNotifierAlarm(handle.get_handle(), trigger_time) ]
    }

    pub fn stop_notifier_alarm(handle: NotifierHandle) -> HalResult<()> {
        hal_call![ ptr HAL_StopNotifierAlarm(handle.get_handle()) ]
    }
}

pub mod pdp {
    pub fn initialize_pdp(module: i32) -> HalResult<()> {
        hal_call![ ptr HAL_InitializePDP(module) ]
    }

    pub fn check_pdp_channel(channel: i32) -> bool {
        unsafe { HAL_CheckPDPChannel(channel) != 0 }
    }

    pub fn check_pdp_module(module: i32) -> bool {
        unsafe { HAL_CheckPDPModule(module) != 0 }
    }

    pub fn get_pdp_temperature(module: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPTemperature(module) ]
    }

    pub fn get_pdp_voltage(module: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPVoltage(module) ]
    }

    pub fn get_pdp_channel_current(module: i32, channel: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPChannelCurrent(module, channel) ]
    }

    pub fn get_pdp_total_current(module: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPTotalCurrent(module) ]
    }

    pub fn get_pdp_total_power(module: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPTotalPower(module) ]
    }

    pub fn get_pdp_total_energy(module: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPDPTotalEnergy(module) ]
    }

    pub fn reset_pdp_total_energy(module: i32) -> HalResult<()> {
        hal_call![ ptr HAL_ResetPDPTotalEnergy(module) ]
    }

    pub fn clear_pdp_sticky_faults(module: i32) -> HalResult<()> {
        hal_call![ ptr HAL_ClearPDPStickyFaults(module) ]
    }
}

pub mod pwm {
    pub fn initialize_pwm_port(handle: PortHandle) -> HalResult<DigitalHandle> {
        hal_call![ ptr HAL_InitializePWMPort(handle.get_handle()) ].map(|n| DigitalHandle(n))
    }

    pub fn free_pwm_port(handle: DigitalHandle) -> HalResult<()> {
        hal_call![ ptr HAL_FreePWMPort(handle.get_handle()) ]
    }

    pub fn check_pwm_channel(channel: i32) -> bool {
        unsafe { HAL_CheckPWMChannel(channel) != 0 }
    }

    pub struct PwmConfig {
        max_pwm: i32,
        deadband_max_pwm: i32,
        center_pwm: i32,
        deadband_min_pwm: i32,
        min_pwm: i32
    }

    pub fn set_pwm_config(handle: DigitalHandle, cfg: PwmConfig) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMConfig(handle.get_handle(), cfg.max_pwm, cfg.deadband_max_pwm, cfg.center_pwm, cfg.deadband_min_pwm, cfg.min_pwm) ]
    }

    pub fn set_pwm_config_raw(handle: DigitalHandle, cfg: PwmConfig) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMConfigRaw(handle.get_handle(), cfg.max_pwm, cfg.deadband_max_pwm, cfg.center_pwm, cfg.deadband_min_pwm, cfg.min_pwm) ]
    }

    pub fn get_pwm_config_raw(handle: DigitalHandle) -> HalResult<PwmConfig> {
        use std::mem;

        // Create a zeroed struct. Will either be filled, or an Err will be returned and cfg will be dropped
        let cfg = PwmConfig {
            max_pwm: mem::zeroed(),
            deadband_max_pwm: mem::zeroed(),
            center_pwm: mem::zeroed(),
            deadband_min_pwm: mem::zeroed(),
            min_pwm: mem::zeroed()
        };

        hal_call![ ptr HAL_GetPWMConfigRaw(
        handle.get_handle(),
        &mut cfg.max_pwm as *mut i32,
        &mut cfg.deadband_max_pwm as *mut i32,
        &mut cfg.center_pwm as *mut i32,
        &mut cfg.deadband_min_pwm as *mut i32,
        &mut cfg.min_pwm as *mut i32
    ) ]
    }

    pub fn set_pwm_eliminate_deadband(handle: DigitalHandle, eliminate_deadband: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMEliminateDeadband(handle.get_handle(), eliminate_deadband as HAL_Bool) ]
    }

    pub fn get_pwm_eliminate_deadband(handle: DigitalHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetPWMEliminateDeadband(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn set_pwm_raw(handle: DigitalHandle, value: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMRaw(handle.get_handle(), value) ]
    }

    pub fn set_pwm_speed(handle: DigitalHandle, speed: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMSpeed(handle.get_handle(), speed) ]
    }

    pub fn set_pwm_position(handle: DigitalHandle, position: f64) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMPosition(handle.get_handle(), position) ]
    }

    pub fn set_pwm_disabled(handle: DigitalHandle) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMDisabled(handle.get_handle()) ]
    }

    pub fn get_pwm_raw(handle: DigitalHandle) -> HalResult<i32> {
        hal_call![ ptr HAL_GetPWMRaw(handle.get_handle()) ]
    }

    pub fn get_pwm_speed(handle: DigitalHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPWMSpeed(handle.get_handle()) ]
    }

    pub fn get_pwm_position(handle: DigitalHandle) -> HalResult<f64> {
        hal_call![ ptr HAL_GetPWMPosition(handle.get_handle()) ]
    }

    pub fn latch_pwm_zero(handle: DigitalHandle) -> HalResult<()> {
        hal_call![ ptr HAL_LatchPWMZero(handle.get_handle()) ]
    }

    pub fn set_pwm_period_scale(handle: DigitalHandle, squelch_mask: i32) -> HalResult<()> {
        hal_call![ ptr HAL_SetPWMPeriodScale(handle.get_handle(), squelch_mask) ]
    }

    pub fn get_loop_timing() -> HalResult<i32> {
        hal_call![ ptr HAL_GetLoopTiming() ]
    }
}

pub mod ports {
    pub fn get_num_accumulators() -> i32 {
        unsafe { HAL_GetNumAccumulators() }
    }

    pub fn get_num_analog_triggers() -> i32 {
        unsafe { HAL_GetNumAnalogTriggers() }
    }

    pub fn get_num_analog_inputs() -> i32 {
        unsafe { HAL_GetNumAnalogInputs() }
    }

    pub fn get_num_analog_outputs() -> i32 {
        unsafe { HAL_GetNumAnalogOutputs() }
    }

    pub fn get_num_counters() -> i32 {
        unsafe { HAL_GetNumCounters() }
    }

    pub fn get_num_digital_headers() -> i32 {
        unsafe { HAL_GetNumDigitalHeaders() }
    }

    pub fn get_num_pwm_headers() -> i32 {
        unsafe { HAL_GetNumPWMHeaders() }
    }

    pub fn get_num_digital_channels() -> i32 {
        unsafe { HAL_GetNumDigitalChannels() }
    }

    pub fn get_num_pwm_channels() -> i32 {
        unsafe { HAL_GetNumPWMChannels() }
    }

    pub fn get_num_digital_pwm_outputs() -> i32 {
        unsafe { HAL_GetNumDigitalPWMOutputs() }
    }

    pub fn get_num_encoders() -> i32 {
        unsafe { HAL_GetNumEncoders() }
    }

    pub fn get_num_interrupts() -> i32 {
        unsafe { HAL_GetNumInterrupts() }
    }

    pub fn get_num_relay_channels() -> i32 {
        unsafe { HAL_GetNumRelayChannels() }
    }

    pub fn get_num_relay_headers() -> i32 {
        unsafe { HAL_GetNumRelayHeaders() }
    }

    pub fn get_num_pcm_modules() -> i32 {
        unsafe { HAL_GetNumPCMModules() }
    }

    pub fn get_num_solenoid_channels() -> i32 {
        unsafe { HAL_GetNumSolenoidChannels() }
    }

    pub fn get_num_pdp_modules() -> i32 {
        unsafe { HAL_GetNumPDPModules() }
    }

    pub fn get_num_pdp_channels() -> i32 {
        unsafe { HAL_GetNumPDPChannels() }
    }
}

pub mod power {
    pub fn get_vin_voltage() -> HalResult<f64> {
        hal_call![ ptr HAL_GetVinVoltage() ]
    }

    pub fn get_vin_current() -> HalResult<f64> {
        hal_call![ ptr HAL_GetVinCurrent() ]
    }

    pub fn get_user_voltage6v() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserVoltage6V() ]
    }

    pub fn get_user_current6v() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserCurrent6V() ]
    }

    pub fn get_user_active6v() -> HalResult<bool> {
        hal_call![ ptr HAL_GetUserActive6V() ].map(|n| n != 0)
    }

    pub fn get_user_current_faults6v() -> HalResult<i32> {
        hal_call![ ptr HAL_GetUserCurrentFaults6V() ]
    }

    pub fn get_user_voltage5v() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserVoltage5V() ]
    }

    pub fn get_user_current5v() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserCurrent5V() ]
    }

    pub fn get_user_active5v() -> HalResult<bool> {
        hal_call![ ptr HAL_GetUserActive5V() ].map(|n| n != 0)
    }

    pub fn get_user_current_faults5v() -> HalResult<i32> {
        hal_call![ ptr HAL_GetUserCurrentFaults5V() ]
    }

    pub fn get_user_voltage3v3() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserVoltage3V3() ]
    }

    pub fn get_user_current3v3() -> HalResult<f64> {
        hal_call![ ptr HAL_GetUserCurrent3V3() ]
    }

    pub fn get_user_active3v3() -> HalResult<bool> {
        hal_call![ ptr HAL_GetUserActive3V3() ].map(|n| n != 0)
    }

    pub fn get_user_current_faults3v3() -> HalResult<i32> {
        hal_call![ ptr HAL_GetUserCurrentFaults3V3() ]
    }
}

pub mod relay {
    pub fn hal_initialize_relay_port(port_handle: PortHandle, fwd: bool) -> HalResult<RelayHandle> {
        hal_call![ ptr HAL_InitializeRelayPort(port_handle.get_handle(), fwd as HAL_Bool) ].map(From::from)
    }

    pub fn free_relay_port(handle: RelayHandle) {
        unsafe { HAL_FreeRelayPort(handle.get_handle()) }
    }

    pub fn check_relay_channel(channel: i32) -> bool {
        unsafe { HAL_CheckRelayChannel() != 0 }
    }

    pub fn set_relay(handle: RelayHandle, on: bool) -> HalResult<()> {
        hal_call![ ptr HAL_SetRelay(handle.get_handle(), on as HAL_Bool) ]
    }

    pub fn get_relay(handle: RelayHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetRelay(handle.get_handle()) ].map(|n| n != 0)
    }
}

pub mod spi {

    use ::raw::*;

    // TODO: handle thingy?
    pub fn initialize_spi(port: i32) -> HalResult<()> {
        hal_call![ ptr HAL_InitializeSPI(port) ]
    }

    pub fn write_spi(port: i32, data_to_send: &[u8], send_size: i32) -> i32 {
        // trick the compiler; the data we're sending *isn't actually mutable*
        unsafe { HAL_WriteSPI(port, data_to_send.as_ptr() as *mut u8, send_size) }
    }

    pub fn read_spi(port: i32, buffer: &mut [u8], count: i32) -> i32 {
        unsafe { HAL_ReadSPI(port, buffer.as_mut_ptr(), count) }
    }

    pub fn close_spi(port: i32) {
        unsafe { raw::HAL_CloseSPI(port) }
    }

    pub fn set_spi_speed(port: i32, speed: i32) {
        unsafe { HAL_SetSPISpeed(port, speed) }
    }

    pub fn set_spi_opts(port: i32, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
        unsafe { HAL_SetSPIOpts(port, msb_first as HAL_Bool, sample_on_trailing as HAL_Bool, clock_idle_high as HAL_Bool) }
    }

    pub fn set_spi_chip_select_active_high(port: i32) {
        unsafe { HAL_SetSPIChipSelectActiveHigh(port) }
    }

    pub fn set_spi_chip_select_active_low(port: i32) {
        unsafe { HAL_SetSPIChipSelectActiveLow(port) }
    }

    pub fn get_spi_handle(port: i32) -> i32 {
        unsafe { HAL_GetSPIHandle(port) }
    }

    pub fn set_spi_handle(port: i32, handle: i32) {
        unsafe { HAL_SetSPIHandle(port, handle) }
    }

    #[derive(Debug)]
    pub struct SpiAccumulatorOptions {
        period: i32,
        cmd: i32,
        transfer_size: i32,
        valid_mask: i32,
        valid_value: i32,
        data_shift: i32,
        data_size: i32,
        is_signed: bool,
        big_endian: bool
    }

    pub fn init_spi_accumulator(port: i32, opts: SpiAccumulatorOptions) {
        unsafe { HAL_InitSPIAccumulator() }
    }

    pub fn free_spi_accumulator(port: i32) {
        unsafe { HAL_FreeSPIAccumulator() }
    }

    pub fn reset_spi_accumulator(port: i32) {
        unsafe { HAL_ResetSPIAccumulator() }
    }

    pub fn set_spi_accumulator_center(port: i32, center: i32) {
        unsafe { HAL_SetSPIAccumulatorCenter() }
    }

    pub fn set_spi_accumulator_deadband(port: i32, deadband: i32) {
        unsafe { HAL_SetSPIAccumulatorDeadband() }
    }

    pub fn hal_get_spi_accumulator_last_value(port: i32) -> HalResult<i32> {
        hal_call![ ptr HAL_GetSPIAccumulatorLastValue() ]
    }

    pub fn hal_get_spi_accumulator_value(port: i32) -> HalResult<i64> {
        hal_call![ ptr HAL_GetSPIAccumulatorValue() ]
    }

    pub fn hal_get_spi_accumulator_count(port: i32) -> HalResult<i64> {
        hal_call![ ptr HAL_GetSPIAccumulatorCount() ]
    }

    pub fn hal_get_spi_accumulator_average(port: i32) -> HalResult<f64> {
        hal_call![ ptr HAL_GetSPIAccumulatorAverage() ]
    }

    pub fn get_spi_accumulator_output(port: i32, value: *mut i64, count: *mut i64) {
        unsafe { HAL_GetSPIAccumulatorOutput(port, &mut value) }
    }
}

// One whole big TODO
pub mod serial {
    pub fn initialize_serial_port(port: SerialPort) {
        unsafe { HAL_InitializeSerialPort(port.into()) }
    }

    pub fn set_serial_baud_rate(port: SerialPort, baud: i32) {
        unsafe { HAL_SetSerialBaudRate(port.into(), baud) }
    }

    pub fn set_serial_data_bits(port: SerialPort, bits: i32) {
        unsafe { HAL_SetSerialDataBits(port.into(), bits) }
    }

    // TODO: What is parity?
    pub fn set_serial_parity(port: SerialPort, parity: i32) {
        unsafe { HAL_SetSerialParity(port.into(), parity) }
    }

    pub fn set_serial_stop_bits(port: SerialPort, stop_bits: i32) {
        unsafe { HAL_SetSerialStopBits(port.into(), stop_bits) }
    }

    // TODO: What is "mode"?
    pub fn set_serial_write_mode(port: SerialPort, mode: i32) {
        unsafe { HAL_SetSerialWriteMode(port.into(), mode) }
    }

    // TODO: What is "flow"?
    pub fn set_serial_flow_control(port: SerialPort, flow: i32) {
        unsafe { HAL_SetSerialFlowControl(port.into(), flow) }
    }

    pub fn set_serial_timeout(port: SerialPort, timeout: f64) {
        unsafe { HAL_SetSerialTimeout(port.into(), timeout) }
    }

    pub fn enable_serial_termination(port: SerialPort, terminator: u8) {
        unsafe { HAL_EnableSerialTermination(port.into(), terminator) }
    }

    pub fn disable_serial_termination(port: SerialPort) {
        unsafe { HAL_DisableSerialTermination(port.into()) }
    }

    pub fn set_serial_read_buffer_size(port: SerialPort, size: i32) {
        unsafe { HAL_SetSerialReadBufferSize(port.into(), size) }
    }

    pub fn set_serial_write_buffer_size(port: SerialPort, size: i32) {
        unsafe { HAL_SetSerialWriteBufferSize(port.into(), size) }
    }

    pub fn get_serial_bytes_received(port: SerialPort) -> HalResult<i32> {
        hal_call![ ptr HAL_GetSerialBytesReceived(port.into()) ]
    }

    pub fn read_serial(port: SerialPort, buffer: &mut [u8], count: i32) -> HalResult<i32> {
        // The RoboRIO is ARM, so we really only need to support ARM architecture. c_char is u8 on ARM.
        // We can't mutate a string slice because the C lib isn't required to return valid utf-8
        // sequences, so we just store it in a u8 slice
        // TODO: Maybe make this a bit more robust?
        hal_call![ ptr HAL_ReadSerial(port.into(), buffer.as_mut_ptr(), count) ]
    }

    pub fn write_serial(port: SerialPort, buffer: &[u8], count: i32) -> HalResult<i32> {
        hal_call![ ptr HAL_WriteSerial(port.into(), buffer.as_ptr(), count) ]
    }

    pub fn flush_serial(port: SerialPort) {
        unsafe { HAL_FlushSerial(port.into()) }
    }

    pub fn clear_serial(port: SerialPort) {
        unsafe { HAL_ClearSerial(port.into()) }
    }

    pub fn close_serial(port: SerialPort) {
        unsafe { HAL_CloseSerial(port.into()) }
    }
}

pub mod solenoid {
    pub fn initialize_solenoid_port(handle: PortHandle) -> HalResult<SolenoidHandle> {
        hal_call![ ptr HAL_InitializeSolenoidPort(handle.get_handle()) ].map(|n| SolenoidHandle(n))
    }

    pub fn free_solenoid_port(handle: SolenoidHandle) {
        unsafe { HAL_FreeSolenoidPort(handle.get_handle()) }
    }

    pub fn check_solenoid_module(module: i32) -> bool {
        unsafe { HAL_CheckSolenoidModule(module) != 0 }
    }

    pub fn check_solenoid_channel(channel: i32) -> bool {
        unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
    }

    pub fn get_solenoid(handle: SolenoidHandle) -> HalResult<bool> {
        hal_call![ ptr HAL_GetSolenoid(handle.get_handle()) ].map(|n| n != 0)
    }

    pub fn get_all_solenoids(module: i32) -> HalResult<i32> {
        hal_call![ ptr HAL_GetAllSolenoids() ]
    }

    pub fn set_solenoid(solenoid_port_handle: SolenoidHandle, value: bool) {
        unsafe { HAL_SetSolenoid(solenoid_port_handle.get_handle(), value as HAL_Bool) }
    }

    pub fn set_all_solenoids(module: i32, state: i32) {
        unsafe { HAL_SetAllSolenoids(module, state) }
    }

    pub fn get_pcm_solenoid_black_list(module: i32) -> HalResult<i32> {
        hal_call![ ptr HAL_GetPCMSolenoidBlackList(module) ]
    }

    pub fn get_pcm_solenoid_voltage_sticky_fault(module: i32) -> HalResult<bool> {
        hal_call![ ptr HAL_GetPCMSolenoidVoltageStickyFault(module) ].map(|n| n != 0)
    }

    pub fn get_pcm_solenoid_voltage_fault(module: i32) -> HalResult<bool> {
        hal_call![ ptr HAL_GetPCMSolenoidVoltageFault(module) ].map(|n| n != 0)
    }

    pub fn clear_all_pcm_sticky_faults(module: i32) {
        unsafe { HAL_ClearAllPCMStickyFaults(module) }
    }
}

pub fn get_error_message(code: i32) -> String {
    let char_ptr = unsafe { HAL_GetErrorMessage(code) };
    CStr::from_ptr(char_ptr).to_string_lossy().into_owned()
}

pub fn get_fpga_version() -> HalResult<i32> {
    hal_call![ ptr HAL_GetFPGAVersion() ]
}

pub fn get_fpga_revision() -> HalResult<i64> {
    hal_call![ ptr HAL_GetFPGARevision() ]
}

pub fn get_runtime_type() -> RuntimeType {
    RuntimeType::from(unsafe { HAL_GetRuntimeType() })
}

pub fn get_fpga_button() -> HalResult<bool> {
    hal_call![ ptr HAL_GetFPGAButton() ].map(|n| n != 0)
}

pub fn get_system_active() -> HalResult<bool> {
    hal_call![ ptr HAL_GetSystemActive() ].map(|n| n != 0)
}

pub fn get_browned_out() -> HalResult<bool> {
    hal_call![ ptr HAL_GetBrownedOut() ].map(|n| n != 0)
}

pub fn base_initialize() {
    unsafe { HAL_BaseInitialize() }
}

pub fn get_port(channel: i32) -> PortHandle {
    PortHandle(unsafe { HAL_GetPort(channel) })
}

pub fn get_port_with_module(module: i32, channel: i32) -> PortHandle {
    PortHandle(unsafe { HAL_GetPortWithModule(module, channel) })
}

pub fn get_fpga_time() -> HalResult<u64> {
    hal_call![ ptr HAL_GetFPGATime() ]
}

/// Initialize the HAL
pub fn hal_initialize(mode: i32) -> i32 {
    unsafe { HAL_Initialize(mode) }
}

pub fn report(resource: i32, instance_number: i32, context: i32, feature: &[u8]) -> i64 {
    unsafe { HAL_Report(resource, instance_number, context, feature.as_ptr()) }
}
