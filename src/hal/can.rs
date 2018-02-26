use error::*;
use std::os::raw::c_float;

extern "C" {
	fn FRC_NetworkCommunication_CANSessionMux_sendMessage(message_id: u32, data: *const u8, data_size: u8, period_ms: i32, status: *mut i32);
	fn FRC_NetworkCommunication_CANSessionMux_receiveMessage(message_id: u32, message_id_mask: u32, data: *mut u8, data_size: *mut u8, timestamp: *mut u32, status: *mut i32);
    fn FRC_NetworkCommunication_CANSessionMux_openStreamSession(session_handle: *mut u32, message_id: u32, message_id_mask: u32, max_messages: u32, status: *mut i32);
	fn FRC_NetworkCommunication_CANSessionMux_closeStreamSession(session_handle: u32);
	fn FRC_NetworkCommunication_CANSessionMux_readStreamSession(session_handle: u32, messages: *mut CanStreamMessage, messages_to_read: u32, messages_read: *mut u32, status: *mut i32);
	fn FRC_NetworkCommunication_CANSessionMux_getCANStatus(percent_bus_utilization: *mut c_float, bus_off_count: *mut u32, tx_full_count: *mut u32, receive_error_count: *mut u32, transmit_error_count: *mut u32, status: *mut i32);
}

#[repr(C)]
pub struct CanStreamMessage {
	pub message_id: u32,
	pub timestamp: u32,
	pub data: [u8; 8],
	pub data_size: u8,
}

pub fn send_message(id: u32, data: &[u8], period_md: i32) -> HalResult<()> {

}

pub fn receive_message(id: u32, id_mask: u32) -> HalResult<()> {

}
