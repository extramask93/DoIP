use byteorder::{BigEndian, ByteOrder};

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, FromPrimitive, ToPrimitive)]
pub enum NackCode {
    #[default]
    IncorrectPattern = 0x0,
    UnknownPayloadType = 0x1,
    MessageTooLong = 0x2,
    OutOfMemory = 0x3,
    InvalidPayloadLength = 0x4,
    /*0x5 - 0xFF Reserved by 13400*/
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, Default)]
pub enum ProtocolVersion {
    /*0x0 - Reserved */
    ISO13400_2010 = 0x1,
    ISO13400_2012 = 0x2,
    ISO13400_2019 = 0x3,
    /*0x4 - 0xFE Reserved by 13400*/
    #[default]
    Default = 0xFF,
}
#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Debug, FromPrimitive, ToPrimitive, Default)]
pub enum PayloadType {
    #[default]
    HeaderNack = 0x0,
    VehicleIDReq = 0x1,
    VehicleIDReqByEID = 0x2,
    VehicleIDReqByVIN = 0x3,
    VehicleIDRes = 0x4,
    RoutingActivationReq = 0x5,
    RoutingActivationRes = 0x6,
    AliveCheckReq = 0x7,
    AliveCheckRes = 0x8,
    /*0x9 - 0x4000 Reserved by 13400*/
    EntityStatusReq = 0x4001,
    EntityStatusRes = 0x4002,
    DiagPowerModeReq = 0x4003,
    DiagPowerModeRes = 0x4004,
    /*0x4005 - 0x8000 Reserved by 13400*/
    DiagMessage = 0x8001,
    DiagMessageAck = 0x8002,
    DiagMessageNAck = 0x8003,
    /*0x8004 - 0xEFFF Reserved by 13400*/
    /*0xF000 - 0xFFFF Reserved for manufacturer*/
}

#[derive(Default)]
pub struct DoIPHeader {
    pub protocol_version: ProtocolVersion,
    pub payload_type: PayloadType,
    pub payload_length: u32,
}
impl DoIPHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::new();
        let mut convert_buf: [u8; 4] = [0; 4];
        buf.push(num::ToPrimitive::to_u8(&self.protocol_version).unwrap());
        buf.push(!num::ToPrimitive::to_u8(&self.protocol_version).unwrap());
        BigEndian::write_u16(
            &mut convert_buf,
            num::ToPrimitive::to_u16(&self.payload_type).unwrap(),
        );
        buf.extend_from_slice(&convert_buf[0..2]);
        BigEndian::write_u32(&mut convert_buf, self.payload_length);
        buf.extend_from_slice(&convert_buf[0..4]);
        buf
    }
    pub const fn length() -> usize {
        8
    }
    pub fn get_payload_len(buffer: &[u8]) -> u32 {
        if buffer.len() < DoIPHeader::length() {
            return 0;
        }
        return BigEndian::read_u32(&buffer[4..8]);
    }
    pub fn from_buffer(buffer: &[u8]) -> Result<DoIPHeader, NackCode> {
        if buffer.len() < DoIPHeader::length() {
            return Err(NackCode::InvalidPayloadLength);
        }
        let protocol_version: Option<ProtocolVersion> = num::FromPrimitive::from_u8(buffer[0]);
        let protocol_version_byte = buffer[0];
        let inverted_protocol_version = buffer[1];
        if protocol_version.is_none() || protocol_version_byte ^ inverted_protocol_version != 0xFF {
            return Err(NackCode::IncorrectPattern);
        }

        let payload_type_native: u16 = BigEndian::read_u16(&buffer[2..4]);
        let payload_type: PayloadType = match num::FromPrimitive::from_u16(payload_type_native) {
            Some(a) => a,
            None => return Err(NackCode::UnknownPayloadType),
        };
        let payload_length = BigEndian::read_u32(&buffer[4..8]);
        Ok(DoIPHeader {
            protocol_version: protocol_version.unwrap(),
            payload_type,
            payload_length,
        })
    }
}
