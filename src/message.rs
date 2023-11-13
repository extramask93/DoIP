pub mod diag_message; //expose to parent module by using pub
pub mod header;
pub mod routing_activation;
pub mod vehicle_identification;
pub mod alive_check;
use crate::message::diag_message::DiagMessage;
use crate::message::header::{DoIPHeader, PayloadType};
use crate::message::header::NackCode;
use crate::message::routing_activation::{RoutingActivationRequest, RoutingActivationResponse};
use crate::message::vehicle_identification::{
    VehicleIdentificationRequest, VehicleIdentificationRequestEID, VehicleIdentificationRequestVIN,
};
use crate::message::alive_check::{AliveCheckRequest, AliveCheckResponse};
use crate::message::entity_status::{EntityStatusRequest, EntityStatusResponse};

pub trait Message {
    /*Move it to the header*/
    fn deserialize(&mut self, header: &DoIPHeader, payload: &[u8]) -> Result<(), NackCode>;
    fn serialize(&self);
}

pub fn message_factory(header: &DoIPHeader, payload: &[u8]) -> Result<Box<dyn Message>, NackCode> {
    let mut message: Box<dyn Message> = match header.payload_type {
        //PayloadType::HeaderNack => Box::new(HeaderNack),
        //PayloadType::VehicleIDRes => Box::new(VehicleIDRes),
        PayloadType::VehicleIDReq => Box::new(VehicleIdentificationRequest::default()),
        PayloadType::VehicleIDReqByEID => Box::new(VehicleIdentificationRequestEID::default()),
        PayloadType::VehicleIDReqByVIN => Box::new(VehicleIdentificationRequestVIN::default()),
        PayloadType::RoutingActivationReq => Box::new(RoutingActivationRequest::default()),
        PayloadType::RoutingActivationRes => Box::new(RoutingActivationResponse::default()),
        PayloadType::AliveCheckReq => Box::new(AliveCheckRequest::default()),
        PayloadType::AliveCheckRes => Box::new(AliveCheckResponse::default()),
        PayloadType::EntityStatusReq => Box::new(EntityStatusRequest::default()),
        PayloadType::EntityStatusRes => Box::new(EntityStatusResponse::default()),
        PayloadType::DiagMessage => Box::new(DiagMessage::default()),
        //PayloadType::DiagPowerModeReq => Box::new(DiagPowerModeReq),
        //PayloadType::DiagPowerModeRes => Box::new(DiagPowerModeRes),
        //PayloadType::DiagMessageAck => Box::new(DiagMessageAck),
        //PayloadType::DiagMessageNAck => Box::new(DiagMessageNAck),
        _ => Box::new(DiagMessage::default()),
    };
    message.deserialize(header, payload)?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_factory() {}
}
