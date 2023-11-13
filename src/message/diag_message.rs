use crate::message::header::DoIPHeader;
use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};
use std::cmp::max;

#[derive(Default)]
pub struct DiagMessage {
    source_address: u16,
    target_address: u16,
    user_data: Vec<u8>
}
impl Message for DiagMessage {
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode> {
        if payload.len() < max(header.payload_length as usize , 4) {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        self.target_address = BigEndian::read_u16(&payload[2..4]);
        self.user_data = payload[4..].to_vec();
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
