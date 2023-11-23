use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct RoutingActivationRequest {
    source_address: u16,
    activation_type: u8,
    reserved_doc: u32,
    reserved_vm: Option<u32>
}
impl RoutingActivationRequest {
    pub fn from_payload(payload: &[u8], expected_len: usize) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload, expected_len)?;
        Ok(s)
    }
}
impl Message for RoutingActivationRequest  {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if ![7,11].contains(&expected_len) 
        || payload.len() < expected_len {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.source_address = BigEndian::read_u16(&payload[0..2]);
        self.activation_type = payload[2];
        self.reserved_doc = BigEndian::read_u32(&payload[3..7]);
        if payload.len() == 11 {
            self.reserved_vm = Some(BigEndian::read_u32(&payload[7..11]));
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
#[derive(Default)]
pub struct RoutingActivationResponse {
    client_logical_address: u16,
    entity_logical_address: u16,
    routing_activation_response_code: u8,
    reserved_doc: u32,
    reserved_vm: Option<u32>
}
impl RoutingActivationResponse {
    pub fn from_payload(payload: &[u8], expected_len: usize) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload, expected_len)?;
        Ok(s)
    }
}
impl Message for RoutingActivationResponse  {
    fn deserialize(&mut self,payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if ![9,13].contains(&expected_len) {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.client_logical_address = BigEndian::read_u16(&payload[0..2]);
        self.entity_logical_address = BigEndian::read_u16(&payload[2..4]);
        self.routing_activation_response_code = payload[4];
        self.reserved_doc = BigEndian::read_u32(&payload[5..9]);
        if payload.len() == 13 {
            self.reserved_vm = Some(BigEndian::read_u32(&payload[7..13]));
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
