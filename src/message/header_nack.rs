use crate::message::header::NackCode;
use crate::message::Message;

use super::header::{DoIPHeader, PayloadType, ProtocolVersion};

#[derive(Default)]
pub struct HeaderNackMessage {
    nack_code: NackCode
}
impl HeaderNackMessage {
    pub fn from_payload(payload: &[u8], expected_len: usize) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload, expected_len)?;
        Ok(s)
    }
    pub fn from_nack_code(code: NackCode) -> Self {
        HeaderNackMessage { nack_code: code}
    }
}
impl Message for HeaderNackMessage {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 1 || payload.is_empty() {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.nack_code = num::FromPrimitive::from_u8(payload[0]).unwrap();
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut header: DoIPHeader = Default::default();
        header.payload_length = 1;
        header.payload_type = PayloadType::HeaderNack;
        header.protocol_version = ProtocolVersion::ISO13400_2019;
        let mut buf: Vec<u8> = header.serialize();
        buf.push(num::ToPrimitive::to_u8(&self.nack_code).unwrap());
        buf
    }
}
