use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

use super::header::DoIPHeader;

#[derive(Default)]
pub struct AliveCheckRequest {}
impl AliveCheckRequest {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for AliveCheckRequest {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length > 0  {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        Vec::<u8>::default()
    }
}
#[derive(Default)]
pub struct AliveCheckResponse {
    pub source_address: u16,
}
impl AliveCheckResponse {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for AliveCheckResponse {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if payload.len() < 2 || header.payload_length != 2  {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        result.resize(2, 0);
        BigEndian::write_u16(&mut result, self.source_address);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alive_check_request() {
        let mut message = AliveCheckRequest::default();
    }
    #[test]
    fn test_alive_check_request_fail() {
    }
    #[test]
    fn test_alive_check_response_ok() {
    }
}
