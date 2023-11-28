use crate::message::{
    header::{DoIPHeader, NackCode},
    message_factory,
    vehicle_identification::{FurtherAction, VehicleIdentificationResponse},
    Message, MessageVariant,
};
use rand::Rng;
use std::{
    io::{self},
    net::{TcpListener, TcpStream, UdpSocket, Ipv4Addr, SocketAddr},
    thread::{self},
    time::Duration,
};

#[derive(Default)]
pub struct DoIPServer {
    vin: [u8; 17],
    eid: [u8; 6],
    gid: [u8; 6],
    logical_address: u16,
}

impl DoIPServer {
    const DOIP_PORT: u16 = 13200;
    const A_DO_IP_ANNOUNCE_NUM: u8 = 3;
    const A_DO_IP_ANNOUNCE_INTERVAL: Duration = Duration::from_millis(500);
    const T_TCP_GENERAL_INACTIVITY: Duration = Duration::from_secs(5 * 60);
    const T_TCP_INITIAL_INACTIVITY: Duration = Duration::from_secs(2);
    const T_TCP_ALIVE_CHECK: Duration = Duration::from_millis(500);

    fn handle_connection(&self, mut _stream: &TcpStream) {
        todo!();
    }
    pub fn start(&self) {
        let announcement_message: VehicleIdentificationResponse =
            VehicleIdentificationResponse::new(
                &self.vin,
                self.logical_address,
                &self.eid,
                &self.gid,
                FurtherAction::NoFurtherAction,
        );
        let handle = thread::spawn(move||{
            DoIPServer::identification_handler(announcement_message);}
            );
        let listener = TcpListener::bind(("127.0.0.0", DoIPServer::DOIP_PORT)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(&stream),
                Err(_) => eprint!("Invalid stream received"),
            }
        }
        handle.join();
    }
    fn announce_on_upd_socket(
        socket: &UdpSocket,
        response: &VehicleIdentificationResponse,
    ) -> io::Result<()> {
        for _ in 0..DoIPServer::A_DO_IP_ANNOUNCE_NUM {
            DoIPServer::send_announcement(&socket, response)?;
            thread::sleep(DoIPServer::A_DO_IP_ANNOUNCE_INTERVAL);
        }
        Ok(())
    }
    fn send_announcement(
        socket: &UdpSocket,
        response: &VehicleIdentificationResponse,
    ) -> io::Result<()> {
        socket.send_to(&response.serialize(), (Ipv4Addr::BROADCAST, DoIPServer::DOIP_PORT))?;
        Ok(())
    }
    fn announce_wait_random() {
        let a_do_ip_announce_wait: Duration =
            Duration::from_millis(rand::thread_rng().gen_range(0..=500));
        thread::sleep(a_do_ip_announce_wait);
    }
    fn init_udp_socket() -> io::Result<UdpSocket> {
        let socket: UdpSocket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DoIPServer::DOIP_PORT))?;
        socket.set_broadcast(true)?;
        socket.set_write_timeout(Some(Duration::from_secs(5)))?;
        Ok(socket)
    }
    fn is_identification_message_addressed_to_us(
        message: &MessageVariant,
        response: &VehicleIdentificationResponse,
    ) -> bool {
        match message {
            MessageVariant::VehicleIDReqVariant(_) => true,
            MessageVariant::VehicleIDReqByEIDVariant(request) => request.eid == response.eid,
            MessageVariant::VehicleIDReqByVINVariant(request) => request.vin == response.vin,
            _ => false,
        }
    }
    fn parse_identification_request(buff: &[u8], len: usize) -> Result<MessageVariant, NackCode> {
        let header = DoIPHeader::from_buffer(&buff[0..len])?;
        message_factory(&header, &buff[DoIPHeader::length()..])
    }
    fn identification_handler(response: VehicleIdentificationResponse) -> ! {
        let mut header_buff: [u8; 40] = [0; 40];
        let socket = DoIPServer::init_udp_socket().expect("UDP socket setup failed");
        DoIPServer::announce_wait_random();
        DoIPServer::announce_on_upd_socket(&socket, &response).expect("Announcement failed");
        loop {
            if let Ok((len, addr)) = socket.recv_from(&mut header_buff) {
                match DoIPServer::parse_identification_request(&header_buff, len) {
                    Ok(message) => {
                        if DoIPServer::is_identification_message_addressed_to_us(
                            &message, &response,
                        ) {
                            if let Err(r) = socket.send_to(&response.serialize(), addr) {
                                eprintln!("Error during sending announcement: {}", r);
                            }
                        }
                    }
                    Err(code) => {
                        eprintln!("Identification message parsing failed: {:?}", code);
                        //message format wrong, do not send nack, as it could
                        //clog the system
                    }
                }
            } else {
                eprintln!("Error during recv_from");
            }
        } // loop
    }
}

pub struct DoIPServerBuilder {
    server: DoIPServer,
}
impl DoIPServerBuilder {
    pub fn new() -> Self {
        let server: DoIPServer = Default::default();
        DoIPServerBuilder { server }
    }
    pub fn set_vin(&mut self, vin: &[u8; 17]) -> &mut Self {
        self.server.vin.clone_from_slice(vin);
        self
    }
    pub fn set_eid(&mut self, eid: &[u8; 6]) -> &mut Self {
        self.server.eid.clone_from_slice(eid);
        self
    }
    pub fn set_gid(&mut self, gid: &[u8; 6]) -> &mut Self {
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
        let vin: [u8; 17] = [0; 17];
        let eid: [u8; 6] = [0; 6];
        let gid: [u8; 6] = [0; 6];
        let logical_address: u16 = 0;
        let port: u16 = 0;
        let mut server_builder: DoIPServerBuilder;
        server_builder
            .set_vin(&vin)
            .set_eid(&eid)
            .set_gid(&gid)
            .set_logical_address(logical_address);
        let server = server_builder.get_server();
        server.start();
    }
}
