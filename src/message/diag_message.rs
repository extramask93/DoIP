use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

use super::header::DoIPHeader;

#[derive(Default)]
pub struct DiagMessage {
    source_address: u16,
    target_address: u16,
    user_data: Vec<u8>
}
impl DiagMessage {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for DiagMessage {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if !(4..=8).contains(&header.payload_length) ||
           payload.len() < header.payload_length as usize {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        self.target_address = BigEndian::read_u16(&payload[2..4]);
        if header.payload_length > 4  {
            self.user_data.extend_from_slice(&payload[4.. header.payload_length as usize])
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8>{
        todo!()
    }
}
#[repr(u8)]
#[derive(Default, FromPrimitive)]
enum AckCode {
    #[default]
    Ack = 0x16,
}
#[derive(Default)]
pub struct DiagMessageAck {
    source_address: u16,
    target_address: u16,
    ack_code: AckCode,
    prev_diag_data: Vec<u8>
}
impl DiagMessageAck {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload )?;
        Ok(s)
    }
}
impl Message for DiagMessageAck {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if !(5..=10).contains(&header.payload_length) ||
           payload.len() < header.payload_length as usize {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        self.target_address = BigEndian::read_u16(&payload[2..4]);
        self.ack_code = num::FromPrimitive::from_u8(payload[4]).unwrap();
        if header.payload_length > 5 {
            self.prev_diag_data.extend_from_slice(&payload[5..header.payload_length as usize]);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Default)]
pub struct DiagMessageNAck {
    source_address: u16,
    target_address: u16,
    nack_code: NackCode,
    prev_diag_data: Vec<u8>
}
impl DiagMessageNAck {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for DiagMessageNAck {
    fn deserialize(&mut self,payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if !(5..=10).contains(&header.payload_length) ||
           payload.len() < header.payload_length as usize {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        self.target_address = BigEndian::read_u16(&payload[2..4]);
        self.nack_code = num::FromPrimitive::from_u8(payload[4]).unwrap();
        if header.payload_length > 5 {
            self.prev_diag_data.extend_from_slice(&payload[5..header.payload_length as usize]);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
