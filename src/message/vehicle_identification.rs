use crate::message::header::NackCode;
use crate::message::Message;
use byteorder::{BigEndian, ByteOrder};

#[derive(Default)]
pub struct VehicleIdentificationRequest {}
impl Message for VehicleIdentificationRequest  {
    fn deserialize(&mut self, _payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len > 0  {
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
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 6 || payload.len() < expected_len {
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
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 17 || payload.len() < expected_len {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.vin.clone_from_slice(&payload[0..17]);
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
/*There are reserved fields in the ISO 13400,
* So any reserved value would crash in the deserialize
* Maybe change the type to something like c_enum*/
#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, Default)]
pub enum FurtherAction {
    #[default]
    NoFurtherAction = 0x0,
    RoutingActivationRequired = 0x10,
}


#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, Default)]
pub enum SyncStatus {
    #[default]
    Synchronized = 0x0,
    Incomplete = 0x10,
}

 
#[derive(Default)]
pub struct VehicleIdentificationResponse
{
    vin: [u8;17],
    logical_address: u16,
    eid: [u8;6],
    gid: [u8;6],
    further_action_required: FurtherAction,
    sync_status: Option<SyncStatus>
}
impl Message for VehicleIdentificationResponse  {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if ![32,33].contains(&expected_len)|| 
           payload.len() < expected_len
        {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.vin.copy_from_slice(&payload[0..17]);
        self.logical_address = BigEndian::read_u16(&payload[17..19]);
        self.eid.copy_from_slice(&payload[19..25]);
        self.gid.copy_from_slice(&payload[25..31]);
        self.further_action_required = num::FromPrimitive::from_u8(payload[31]).unwrap();
        if expected_len == 33 {
            self.sync_status = Some(num::FromPrimitive::from_u8(payload[32]).unwrap());
        }
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
