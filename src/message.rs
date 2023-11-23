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

pub fn message_factory(header: &DoIPHeader, payload: &[u8]) -> Result<MessageVariant, NackCode> {
    let message = match header.payload_type {
        PayloadType::HeaderNack => MessageVariant::HeaderNackMessageVariant(
            HeaderNackMessage::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::VehicleIDRes => MessageVariant::VehicleIDResVariant(
            VehicleIdentificationResponse::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::VehicleIDReq => MessageVariant::VehicleIDReqVariant(
            VehicleIdentificationRequest::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::VehicleIDReqByEID => MessageVariant::VehicleIDReqByEIDVariant(
            VehicleIdentificationRequestEID::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::VehicleIDReqByVIN =>  MessageVariant::VehicleIDReqByVINVariant(
            VehicleIdentificationRequestVIN::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::RoutingActivationReq => MessageVariant::RoutingActivationRequestVariant(
            RoutingActivationRequest::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::RoutingActivationRes => MessageVariant::RoutingActivationResponseVariant(
            RoutingActivationResponse::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::AliveCheckReq => MessageVariant::AliveCheckRequestVariant(
            AliveCheckRequest::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::AliveCheckRes => MessageVariant::AliveCheckRespnseVariant(
            AliveCheckResponse::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::EntityStatusReq => MessageVariant::EntityStatusRequestVariant(
            EntityStatusRequest::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::EntityStatusRes => MessageVariant::EntityStatusResponseVariant(
            EntityStatusResponse::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::DiagPowerModeReq => MessageVariant::DiagnosticPowerModeRequestVariant(
            DiagnosticPowerModeRequest::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::DiagPowerModeRes => MessageVariant::DiagnosticPowerModeResponseVariant(
            DiagnosticPowerModeResponse::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::DiagMessage => MessageVariant::DiagnoticMessageVariant(
            DiagMessage::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::DiagMessageAck => MessageVariant::DiagnosticMessageAckVariant(
            DiagMessageAck::from_payload(payload, header.payload_length as usize)?
            ),
        PayloadType::DiagMessageNAck => MessageVariant::DiagnosticMessageNAckVariant(
            DiagMessageNAck::from_payload(payload, header.payload_length as usize)?
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
