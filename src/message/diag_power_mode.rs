use crate::message::header::NackCode;
use crate::message::Message;

use super::header::DoIPHeader;

#[derive(Default)]
pub struct DiagnosticPowerModeRequest {
}
impl DiagnosticPowerModeRequest {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for DiagnosticPowerModeRequest {
    fn deserialize(&mut self,payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if  header.payload_length!= 0 {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
#[repr(u8)]
#[derive(FromPrimitive)]
pub enum DiagnosticPowerMode {
    NotReady = 0x0,
    Ready = 0x1,
    NotSupported = 0x2,
    /*Reserved 0x3-0xf*/
}

impl Default for DiagnosticPowerMode {
    fn default() -> Self {
        Self::NotReady
    }
}
#[derive(Default)]
pub struct DiagnosticPowerModeResponse {
    power_mode: DiagnosticPowerMode,
}
impl DiagnosticPowerModeResponse {
    pub fn from_payload(payload: &[u8]) ->Result<Self,NackCode> {
        let mut s = Self::default();
        s.deserialize(payload)?;
        Ok(s)
    }
}
impl Message for DiagnosticPowerModeResponse {
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode> {
        let header = DoIPHeader::from_buffer(payload)?;
        if header.payload_length != 1 || payload.is_empty() {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.power_mode = num::FromPrimitive::from_u8(payload[0]).unwrap();
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
