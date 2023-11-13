use crate::message::header::DoIPHeader;
use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};
use std::cmp::max;

#[derive(Default)]
pub struct VehicleIdentificationRequest {}
impl Message for VehicleIdentificationRequest  {
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
pub struct VehicleIdentificationRequestEID
{
    eid: [u8;6]
}
impl Message for VehicleIdentificationRequestEID  {
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode> {
        if payload.len() != 6 || header.payload_length != 6 {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.eid.clone_from_slice(&payload[0..6]);
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
#[derive(Default)]
pub struct VehicleIdentificationRequestVIN
{
    vin: [u8;17]
}
impl Message for VehicleIdentificationRequestVIN  {
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode> {
        if payload.len() != 17 || header.payload_length != 17 {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.vin.clone_from_slice(&payload[0..17]);
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
