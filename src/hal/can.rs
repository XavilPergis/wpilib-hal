use ::raw::*;

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

