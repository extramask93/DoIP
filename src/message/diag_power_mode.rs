use crate::message::header::NackCode;
use crate::message::Message;

#[derive(Default)]
pub struct DiagnosticPowerModeRequest {
}
impl Message for DiagnosticPowerModeRequest {
    fn deserialize(&mut self,_payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 0 {
            return Err(NackCode::InvalidPayloadLength);
        }
        Ok(())
    }

    fn serialize(&self) {
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
impl Message for DiagnosticPowerModeResponse {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 1 || payload.is_empty() {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.power_mode = num::FromPrimitive::from_u8(payload[0]).unwrap();
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
