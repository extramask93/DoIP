use crate::message::header::DoIPHeader;
use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};
use std::cmp::max;
#[derive(Default)]
pub struct AliveCheckRequest {}
impl Message for AliveCheckRequest {
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode> {
        if payload.len() > 0 || header.payload_length > 0  {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
#[derive(Default)]
pub struct AliveCheckResponse {
    source_address: u16,
}
impl Message for AliveCheckResponse {
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode> {
        if payload.len() != 2 || header.payload_length != 2  {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
