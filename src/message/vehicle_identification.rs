use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

use super::header::{DoIPHeader, PayloadType, ProtocolVersion};

#[derive(Default)]
pub struct VehicleIdentificationRequest {}
impl VehicleIdentificationRequest {
    pub fn from_payload(payload: &[u8]) -> Result<Self, NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for VehicleIdentificationRequest {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length > 0 {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
#[derive(Default)]
pub struct VehicleIdentificationRequestEID {
    pub eid: [u8; 6],
}
impl VehicleIdentificationRequestEID {
    pub fn from_payload(payload: &[u8]) -> Result<Self, NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for VehicleIdentificationRequestEID {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length != 6
            || payload.len() < header.payload_length as usize + DoIPHeader::length()
        {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.eid.clone_from_slice(&payload[0..6]);
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
#[derive(Default)]
pub struct VehicleIdentificationRequestVIN {
    pub vin: [u8; 17],
}
impl VehicleIdentificationRequestVIN {
    pub fn from_payload(payload: &[u8]) -> Result<Self, NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for VehicleIdentificationRequestVIN {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length != 17
            || payload.len() < DoIPHeader::length() + header.payload_length as usize
        {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.vin.clone_from_slice(&payload[0..17]);
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
/*There are reserved fields in the ISO 13400,
* So any reserved value would crash in the deserialize
* Maybe change the type to something like c_enum*/
#[repr(u8)]
#[derive(Copy, Clone, Debug, ToPrimitive, FromPrimitive, Default)]
pub enum FurtherAction {
    #[default]
    NoFurtherAction = 0x0,
    RoutingActivationRequired = 0x10,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, Default)]
pub enum SyncStatus {
    #[default]
    Synchronized = 0x0,
    Incomplete = 0x10,
}

#[derive(Default, Debug)]
pub struct VehicleIdentificationResponse {
    pub vin: [u8; 17],
    pub logical_address: u16,
    pub eid: [u8; 6],
    pub gid: [u8; 6],
    further_action_required: FurtherAction,
    sync_status: Option<SyncStatus>,
}
impl VehicleIdentificationResponse {
    pub fn from_payload(payload: &[u8]) -> Result<Self, NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
    pub fn new(
        vin: &[u8; 17],
        logical_address: u16,
        eid: &[u8; 6],
        gid: &[u8; 6],
        further_action_required: FurtherAction,
    ) -> Self {
        let mut result = VehicleIdentificationResponse::default();
        result.vin.copy_from_slice(vin);
        result.logical_address = logical_address;
        result.eid.copy_from_slice(eid);
        result.gid.copy_from_slice(gid);
        result.further_action_required = further_action_required;
        result
    }
}
impl Message for VehicleIdentificationResponse {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(&payload)?;
        if ![32, 33].contains(&header.payload_length)
            || payload.len() < header.payload_length as usize
        {
            return Err(NackCode::InvalidPayloadLength);
        }
        let payload = &payload[DoIPHeader::length()..];
        self.vin.copy_from_slice(&payload[0..17]);
        self.logical_address = BigEndian::read_u16(&payload[17..19]);
        self.gid.copy_from_slice(&payload[19..25]);
        self.eid.copy_from_slice(&payload[25..31]);
        self.further_action_required =
            num::FromPrimitive::from_u8(payload[31]).unwrap();
        if payload.len() > 32 {
            self.sync_status = Some(num::FromPrimitive::from_u8(payload[32]).unwrap());
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut conversion_buffer: [u8; 2] = [0; 2];
        let mut header: DoIPHeader = Default::default();
        header.payload_length = match self.sync_status {
            Some(_) => 33,
            None => 32,
        };
        header.payload_type = PayloadType::VehicleIDRes;
        header.protocol_version = ProtocolVersion::ISO13400_2019;
        let mut buf = header.serialize();
        buf.extend_from_slice(&self.vin);
        BigEndian::write_u16(&mut conversion_buffer, self.logical_address);
        buf.extend_from_slice(&conversion_buffer);
        buf.extend_from_slice(&self.eid);
        buf.extend_from_slice(&self.gid);
        buf.push(num::ToPrimitive::to_u8(&self.further_action_required).unwrap());
        if let Some(sync_status) = self.sync_status {
            buf.push(num::ToPrimitive::to_u8(&sync_status).unwrap());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    #[test]
    fn deserialize_vehicle_id_response_invalid_payload_len() {
        let mut response = VehicleIdentificationResponse::default();
        let serialized = response.serialize();
        let result = response.deserialize(&serialized[0..serialized.len() - 1]);
        assert!(result.is_err());
        assert_eq!(result, Err(NackCode::InvalidPayloadLength));
    }
    #[test]
    fn serialize_deserialize_vehicle_id_response() {
        let vin: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let eid: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let gid: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let logical_address: u16 = 69;
        let response = VehicleIdentificationResponse::new(
            &vin.try_into().unwrap(),
            logical_address,
            &eid.try_into().unwrap(),
            &gid.try_into().unwrap(),
            FurtherAction::NoFurtherAction,
        );
        let serialized = response.serialize();
        let mut deserialized = VehicleIdentificationResponse::default();
        assert!(deserialized.deserialize(&serialized).is_ok());
        assert!(deserialized.serialize() == serialized);
    }
}
