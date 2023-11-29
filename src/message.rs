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
    fn deserialize(&mut self, payload: &[u8]) -> Result<(), NackCode>;
    fn serialize(&self) -> Vec<u8>;
}
pub enum MessageVariant {
    HeaderNackMessageVariant(HeaderNackMessage),
    VehicleIDResVariant(VehicleIdentificationResponse),
    VehicleIDReqVariant(VehicleIdentificationRequest),
    VehicleIDReqByEIDVariant(VehicleIdentificationRequestEID),
    VehicleIDReqByVINVariant(VehicleIdentificationRequestVIN),
    RoutingActivationRequestVariant(RoutingActivationRequest),
    RoutingActivationResponseVariant(RoutingActivationResponse),
    AliveCheckRequestVariant(AliveCheckRequest),
    AliveCheckRespnseVariant(AliveCheckResponse),
    EntityStatusRequestVariant(EntityStatusRequest),
    EntityStatusResponseVariant(EntityStatusResponse),
    DiagnoticMessageVariant(DiagMessage),
    DiagnosticPowerModeRequestVariant(DiagnosticPowerModeRequest),
    DiagnosticPowerModeResponseVariant(DiagnosticPowerModeResponse),
    DiagnosticMessageAckVariant(DiagMessageAck),
    DiagnosticMessageNAckVariant(DiagMessageNAck),
}

pub fn message_factory(payload: &[u8]) -> Result<MessageVariant, NackCode> {
    let header = DoIPHeader::from_buffer(&payload)?;
    let message = match header.payload_type {
        PayloadType::HeaderNack => MessageVariant::HeaderNackMessageVariant(
            HeaderNackMessage::from_payload(payload)?
            ),
        PayloadType::VehicleIDRes => MessageVariant::VehicleIDResVariant(
            VehicleIdentificationResponse::from_payload(payload)?
            ),
        PayloadType::VehicleIDReq => MessageVariant::VehicleIDReqVariant(
            VehicleIdentificationRequest::from_payload(payload)?
            ),
        PayloadType::VehicleIDReqByEID => MessageVariant::VehicleIDReqByEIDVariant(
            VehicleIdentificationRequestEID::from_payload(payload)?
            ),
        PayloadType::VehicleIDReqByVIN =>  MessageVariant::VehicleIDReqByVINVariant(
            VehicleIdentificationRequestVIN::from_payload(payload)?
            ),
        PayloadType::RoutingActivationReq => MessageVariant::RoutingActivationRequestVariant(
            RoutingActivationRequest::from_payload(payload)?
            ),
        PayloadType::RoutingActivationRes => MessageVariant::RoutingActivationResponseVariant(
            RoutingActivationResponse::from_payload(payload)?
            ),
        PayloadType::AliveCheckReq => MessageVariant::AliveCheckRequestVariant(
            AliveCheckRequest::from_payload(payload)?
            ),
        PayloadType::AliveCheckRes => MessageVariant::AliveCheckRespnseVariant(
            AliveCheckResponse::from_payload(payload)?
            ),
        PayloadType::EntityStatusReq => MessageVariant::EntityStatusRequestVariant(
            EntityStatusRequest::from_payload(payload)?
            ),
        PayloadType::EntityStatusRes => MessageVariant::EntityStatusResponseVariant(
            EntityStatusResponse::from_payload(payload)?
            ),
        PayloadType::DiagPowerModeReq => MessageVariant::DiagnosticPowerModeRequestVariant(
            DiagnosticPowerModeRequest::from_payload(payload)?
            ),
        PayloadType::DiagPowerModeRes => MessageVariant::DiagnosticPowerModeResponseVariant(
            DiagnosticPowerModeResponse::from_payload(payload)?
            ),
        PayloadType::DiagMessage => MessageVariant::DiagnoticMessageVariant(
            DiagMessage::from_payload(payload)?
            ),
        PayloadType::DiagMessageAck => MessageVariant::DiagnosticMessageAckVariant(
            DiagMessageAck::from_payload(payload)?
            ),
        PayloadType::DiagMessageNAck => MessageVariant::DiagnosticMessageNAckVariant(
            DiagMessageNAck::from_payload(payload)?
            ),

    };
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_factory() {}
}
