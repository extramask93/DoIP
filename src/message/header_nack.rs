use crate::message::header::NackCode;
use crate::message::Message;

#[derive(Default)]
pub struct HeaderNackMessage {
    nack_code: NackCode
}
impl Message for HeaderNackMessage {
    fn deserialize(&mut self, payload: &[u8], expected_len: usize) -> Result<(), NackCode> {
        if expected_len != 1 || payload.is_empty() {
            return Err(NackCode::InvalidPayloadLength);
        }
        self.nack_code = num::FromPrimitive::from_u8(payload[0]).unwrap();
        Ok(())
    }

    fn serialize(&self) {
        todo!()
    }
}
