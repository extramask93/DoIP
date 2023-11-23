use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::time::Duration;
use std::thread;

use doip::message::header::{DoIPHeader, NackCode};
use doip::message::header_nack::HeaderNackMessage;
use doip::message::vehicle_identification::{FurtherAction, VehicleIdentificationResponse};
use doip::message::{message_factory, Message};


impl DoIPServer {
    fn start() {
        let handle = thread::spawn(DoIPServer::identification_handler);
        let listener = TcpListener::bind("127.0.0.0:7878").unwrap();
        for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_) => eprint!("Invalid stream received"),
        }
        handle.join();
    }
    fn identification_handler() {
        let mut header_buff: [u8;DoIPHeader::length()];
        let socket: UdpSocket = UdpSocket::bind("0.0.0.0:udp_discorvery").unwrap();
        socket.set_write_timeout(None);
        socket.set_broadcast(true).unwrap();
        for n in 0..3 {
            socket.send(&ANNOUNCEMENT_MESSAGE.serialize());
        }
        let (len, addr) = socket.recv_from(&mut header_buff).unwrap();
        //todo parse header
    }
}
struct DoIPBuilder {
}
fn handle_connection(mut stream: &TcpStream) {
    match handle_connection_detail(&mut stream) {
        Ok(_) => (),
        Err((code,bytes_to_read)) => match code {
            doip::message::header::NackCode::InvalidPayloadLength => {
                let message = HeaderNackMessage::from_nack_code(code);
                let payload = message.serialize();
                stream.write_all(&payload);
                stream.shutdown(std::net::Shutdown::Both);
            }
            doip::message::header::NackCode::IncorrectPattern => {
                let message = HeaderNackMessage::from_nack_code(code);
                let payload = message.serialize();
                stream.write_all(&payload);
                stream.shutdown(std::net::Shutdown::Both);
            }
            _ => {
                let message = HeaderNackMessage::from_nack_code(code);
                let payload = message.serialize();
                stream.write_all(&payload);
                std::io::copy(&mut std::io::Read::by_ref(&mut stream).take(bytes_to_read as u64), &mut std::io::sink());
            }
        },
    }
}
fn handle_connection_detail(mut stream: &TcpStream) -> Result<(), (NackCode,usize)> {
    let mut header_buff: [u8; 8] = [0; 8];
    stream.read_exact(&mut header_buff).unwrap();
    let header = DoIPHeader::from_buffer(&header_buff)?;
    let mut message_buffer: Vec<u8>;
    message_buffer.resize(header.payload_length as usize, 0);
    stream.read_exact(&mut message_buffer).unwrap();
    let message = message_factory(&header, &message_buffer).unwrap();
    match message {
        doip::message::MessageVariant::HeaderNackMessageVariant(_) => (), //discard the bytes
        doip::message::MessageVariant::VehicleIDResVariant(_) => todo!(),
        doip::message::MessageVariant::VehicleIDReqVariant(_) => todo!(),
        doip::message::MessageVariant::VehicleIDReqByEIDVariant(_) => todo!(),
        doip::message::MessageVariant::VehicleIDReqByVINVariant(_) => todo!(),
        doip::message::MessageVariant::RoutingActivationRequestVariant(_) => todo!(),
        doip::message::MessageVariant::RoutingActivationResponseVariant(_) => todo!(),
        doip::message::MessageVariant::AliveCheckRequestVariant(_) => todo!(),
        doip::message::MessageVariant::AliveCheckRespnseVariant(_) => todo!(),
        doip::message::MessageVariant::EntityStatusRequestVariant(_) => todo!(),
        doip::message::MessageVariant::EntityStatusResponseVariant(_) => todo!(),
        doip::message::MessageVariant::DiagnoticMessageVariant(_) => todo!(),
        doip::message::MessageVariant::DiagnosticPowerModeRequestVariant(_) => todo!(),
        doip::message::MessageVariant::DiagnosticPowerModeResponseVariant(_) => todo!(),
        doip::message::MessageVariant::DiagnosticMessageAckVariant(_) => todo!(),
        doip::message::MessageVariant::DiagnosticMessageNAckVariant(_) => todo!(),
    }
    Ok(())
}
fn listener() {
    }
}
fn main() {
    //open socket
}
