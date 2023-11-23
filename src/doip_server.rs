use std::{thread::{self}, net::{ TcpListener, UdpSocket,  TcpStream}, time::Duration, sync::Arc };
use rand::Rng;
use crate::message::{header::DoIPHeader, vehicle_identification::{VehicleIdentificationResponse, FurtherAction}, header_nack::HeaderNackMessage, Message, message_factory, MessageVariant};

#[derive(Default)]
pub struct DoIPServer {
    vin: [u8;17],
    eid: [u8;6],
    gid: [u8;6],
    logical_address: u16,
}
impl DoIPServer {
    const DOIP_PORT: u16 = 13200;
    const A_DO_IP_ANNOUNCE_NUM: u8 = 3;
    const A_DO_IP_ANNOUNCE_INTERVAL: Duration = Duration::from_millis(500);
    const T_TCP_GENERAL_INACTIVITY: Duration = Duration::from_secs(5*60);
    const T_TCP_INITIAL_INACTIVITY: Duration = Duration::from_secs(2);
    const T_TCP_ALIVE_CHECK: Duration = Duration::from_millis(500);

    fn handle_connection(&self, mut _stream: &TcpStream) {
        todo!();
    }
    fn get_announcement_message(&self) -> VehicleIdentificationResponse
    {
        let announcement_message: VehicleIdentificationResponse = VehicleIdentificationResponse::new(
        &self.vin,
        self.logical_address,
        &self.eid,
        &self.gid,
        FurtherAction::NoFurtherAction,
        );
        announcement_message
    }
    pub fn start(& self) {
        //let foo = Arc::clone(&self);
        //let handle = thread::spawn(|self.clone()| {self.identification_handler();});
        let listener = TcpListener::bind(("127.0.0.0", DoIPServer::DOIP_PORT)).unwrap();
        for stream in listener.incoming() {
        match stream {
            Ok(stream) => self.handle_connection(&stream),
            Err(_) => eprint!("Invalid stream received"),
        }
    }
        //handle.join();
    }

    fn identification_handler(& self) {
        let mut header_buff: [u8;DoIPHeader::length()+17] = [0; DoIPHeader::length()+17];
        let socket: UdpSocket = UdpSocket::bind(("0.0.0.0",DoIPServer::DOIP_PORT)).expect("Can't bind to the broadcast address");
        socket.set_broadcast(true).expect("Can't establish broadcasting");
        socket.set_write_timeout(Some(Duration::from_secs(5)));
        let a_do_ip_announce_wait : Duration = Duration::from_millis(rand::thread_rng().gen_range(0..=500));
        thread::sleep(a_do_ip_announce_wait);
        for _ in 0..DoIPServer::A_DO_IP_ANNOUNCE_NUM {
            match socket.send(&self.get_announcement_message().serialize()) {
                Ok(_) => (),
                Err(err) => eprintln!("Error during sending an anouncement: {}",err.to_string())
            }
            thread::sleep(DoIPServer::A_DO_IP_ANNOUNCE_INTERVAL);
        }
        loop {
            let (len, addr) = socket.recv_from(&mut header_buff).unwrap();
            if len < DoIPHeader::length() {
                continue;
            }
            let header = match DoIPHeader::from_buffer(&header_buff) {
                Ok(header) => header,
                Err(err) => {
                    let message = HeaderNackMessage::from_nack_code(err);
                    socket.send_to(&message.serialize(), addr).unwrap();
                    continue
                },
            };
            let message = message_factory(&header, &header_buff[DoIPHeader::length()..]).unwrap();
            match message {
                MessageVariant::VehicleIDReqVariant(_) => {
                    let message = VehicleIdentificationResponse::new(&self.vin,
                    self.logical_address,&self.eid, &self.gid, FurtherAction::NoFurtherAction);
                    socket.send_to(&message.serialize(), addr).unwrap();
                },
                MessageVariant::VehicleIDReqByEIDVariant(request) => {
                    if request.eid == self.eid {
                    let message = VehicleIdentificationResponse::new(&self.vin,
                    self.logical_address,&self.eid, &self.gid, FurtherAction::NoFurtherAction);
                    socket.send_to(&message.serialize(), addr).unwrap();
                    }
                },
                MessageVariant::VehicleIDReqByVINVariant(request) => {
                    if request.vin == self.vin {
                    let message = VehicleIdentificationResponse::new(&self.vin,
                    self.logical_address,&self.eid, &self.gid, FurtherAction::NoFurtherAction);
                    socket.send_to(&message.serialize(), addr).unwrap();
                    }
                },
                _ => (/*Discard the message*/)
            }
        }
    }
}

pub struct DoIPServerBuilder {
    server: DoIPServer
}
impl DoIPServerBuilder {
    pub fn new() -> Self {
        let server: DoIPServer = Default::default();
        DoIPServerBuilder {server}
    }
    pub fn set_vin(&mut self, vin: &[u8;17]) -> &mut Self {
        self.server.vin.clone_from_slice(vin); 
        self
    }
    pub fn set_eid(&mut self, eid: &[u8; 6]) -> &mut Self {
        self.server.eid.clone_from_slice(eid);
        self
    }
    pub fn set_gid(&mut self,gid: &[u8; 6]) -> &mut Self {
        self.server.gid.clone_from_slice(gid);
        self
    }
    pub fn set_logical_address(&mut self, address: u16) -> &mut Self {
        self.server.logical_address = address;
        self
    }
    pub fn get_server(self) -> DoIPServer {
        self.server
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_server() {
        let vin: [u8;17] = [0;17];
        let eid: [u8;6] = [0;6];
        let gid: [u8;6] = [0;6];
        let logical_address: u16 = 0;
        let port : u16 = 0;
        let mut server_builder: DoIPServerBuilder;
        server_builder.set_vin(&vin).set_eid(&eid).set_gid(&gid).set_logical_address(logical_address);
        let server = server_builder.get_server();
        server.start();
    }
}
