use std::default;

use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct AliveCheckRequest {}
impl AliveCheckRequest {
    pub fn from_payload(payload: &[u8], expected_len: usize) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload, expected_len)?;
        Ok(s)
    }
}
impl Message for AliveCheckRequest {
    fn deserialize(&mut self, _payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len > 0  {
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
    source_address: u16,
}
impl AliveCheckResponse {
    pub fn from_payload(payload: &[u8], expected_len: usize) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload, expected_len)?;
        Ok(s)
    }
}
impl Message for AliveCheckResponse {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if payload.len() < 2 || expected_len != 2  {
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
        assert!(message.deserialize(&[], 0).is_ok());
        assert!(message.deserialize(&[1,2], 0).is_ok());
    }
    #[test]
    fn test_alive_check_request_fail() {
        let mut message = AliveCheckRequest::default();
        assert!(message.deserialize(&[], 1).is_err());
    }
    #[test]
    fn test_alive_check_response_ok() {
        let mut message = AliveCheckResponse::default();
        let result = message.deserialize(&[1,2], 2); 
        assert!(result.is_ok());
        assert!(message.source_address == 0x102);
    }
}
