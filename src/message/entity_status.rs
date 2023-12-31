use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

use super::header::DoIPHeader;
#[repr(u8)]
#[derive(FromPrimitive, Default)]
pub enum NodeType {
    #[default]
    Gateway = 0x0,
    Node = 0x1,
    /*0x2-0xff - reserved*/
}
#[derive(Default)]
pub struct EntityStatusResponse {
    node_type: NodeType,
    max_sockets: u8,
    open_sockets: u8,
    max_data_size: u32
}
impl EntityStatusResponse {
    pub fn new(node_type: NodeType, max_sockets:u8, open_sockets: u8,
               max_data_size: u32) -> Self {
        EntityStatusResponse{node_type, max_sockets, open_sockets, max_data_size}
    }
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for EntityStatusResponse {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length != 7 || payload.len() < 7 {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.node_type = num::FromPrimitive::from_u8(payload[0]).unwrap();
        self.max_sockets = payload[1];
        self.open_sockets = payload[2];
        self.max_data_size = BigEndian::read_u32(&payload[3..7]);
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
#[derive(Default)]
pub struct EntityStatusRequest {
}
impl EntityStatusRequest {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for EntityStatusRequest {
    fn deserialize(&mut self, _payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(_payload)?;
        if  header.payload_length != 0 {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
