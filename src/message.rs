pub mod diag_message; //expose to parent module by using pub
pub mod header;
pub mod routing_activation;
pub mod vehicle_identification;
pub mod alive_check;
pub mod entity_status;
pub mod header_nack;
pub mod diag_power_mode;
use crate::message::diag_message::{DiagMessage, DiagMessageAck, DiagMessageNAck};
use crate::message::diag_power_mode::{DiagnosticPowerModeRequest, DiagnosticPowerModeResponse};
use crate::message::header::{DoIPHeader, PayloadType};
use crate::message::header_nack::HeaderNackMessage;
use crate::message::header::NackCode;
use crate::message::routing_activation::{RoutingActivationRequest, RoutingActivationResponse};
use crate::message::vehicle_identification::{
    VehicleIdentificationRequest, VehicleIdentificationRequestEID,
    VehicleIdentificationRequestVIN, VehicleIdentificationResponse
};
use crate::message::alive_check::{AliveCheckRequest, AliveCheckResponse};
use crate::message::entity_status::{EntityStatusRequest, EntityStatusResponse};

pub trait Message {
    fn deserialize(&mut self, payload: &[u8], expected_length: usize) -> Result<(), NackCode>;
    fn serialize(&self);
}

pub fn message_factory(header: &DoIPHeader, payload: &[u8]) -> Result<Box<dyn Message>, NackCode> {
    let mut message: Box<dyn Message> = match header.payload_type {
        PayloadType::HeaderNack => Box::<HeaderNackMessage>::default(),
        PayloadType::VehicleIDRes => Box::<VehicleIdentificationResponse>::default(),
        PayloadType::VehicleIDReq => Box::<VehicleIdentificationRequest>::default(),
        PayloadType::VehicleIDReqByEID => Box::<VehicleIdentificationRequestEID>::default(),
        PayloadType::VehicleIDReqByVIN => Box::<VehicleIdentificationRequestVIN>::default(),
        PayloadType::RoutingActivationReq => Box::<RoutingActivationRequest>::default(),
        PayloadType::RoutingActivationRes => Box::<RoutingActivationResponse>::default(),
        PayloadType::AliveCheckReq => Box::<AliveCheckRequest>::default(),
        PayloadType::AliveCheckRes => Box::<AliveCheckResponse>::default(),
        PayloadType::EntityStatusReq => Box::<EntityStatusRequest>::default(),
        PayloadType::EntityStatusRes => Box::<EntityStatusResponse>::default(),
        PayloadType::DiagMessage => Box::<DiagMessage>::default(),
        PayloadType::DiagPowerModeReq => Box::<DiagnosticPowerModeRequest>::default(),
        PayloadType::DiagPowerModeRes => Box::<DiagnosticPowerModeResponse>::default(),
        PayloadType::DiagMessageAck => Box::<DiagMessageAck>::default(),
        PayloadType::DiagMessageNAck => Box::<DiagMessageNAck>::default(),
    };
    message.deserialize(payload, header.payload_length as usize)?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_factory() {}
}
