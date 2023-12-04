use crate::message::{
    entity_status::{EntityStatusResponse, NodeType},
    header::{DoIPHeader, NackCode},
    header_nack::HeaderNackMessage,
    message_factory,
    vehicle_identification::{FurtherAction, VehicleIdentificationResponse},
    Message, MessageVariant, routing_activation::{RoutingActivationResponse, RoutingActivationCode},
};
use rand::Rng;
use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    thread::{self},
    time::Duration,
};

#[derive(Default)]
pub struct DoIPServer {
    vin: [u8; 17],
    eid: [u8; 6],
    gid: [u8; 6],
    logical_address: u16,
    max_sockets: u8,
    open_sockets: u8,
    max_data_size: u32,
    client_source_address: Option<u16>
}
enum ConnectionState {
    initialized,
    registered,
}
impl DoIPServer {
    const DOIP_PORT: u16 = 13200;
    const A_DO_IP_ANNOUNCE_NUM: u8 = 3;
    const A_DO_IP_ANNOUNCE_INTERVAL: Duration = Duration::from_millis(500);
    const T_TCP_GENERAL_INACTIVITY: Duration = Duration::from_secs(5 * 60);
    const T_TCP_INITIAL_INACTIVITY: Duration = Duration::from_secs(2);
    const T_TCP_ALIVE_CHECK: Duration = Duration::from_millis(500);
    fn handle_message(&mut self, stream: &mut TcpStream, message: &MessageVariant) {
        match message {
            MessageVariant::RoutingActivationRequestVariant(req) => {
                if self.client_source_address.is_some_and(|addr| 
                   {req.source_address == addr})
                {
                    let response = RoutingActivationResponse::new(
                        req.source_address,
                        self.logical_address,
                        RoutingActivationCode::RoutingActivated
                    );
                    stream.write_all(&response.serialize()).unwrap();
                }
                else {
                    let response = RoutingActivationResponse::new(
                        req.source_address,
                        self.logical_address,
                        RoutingActivationCode::DeniedDifferentSA
                    );
                    stream.write_all(&response.serialize()).unwrap();
                    //close socket TODO
                }
            }
            MessageVariant::AliveCheckRespnseVariant(resp) => {
                self.client_source_address = Some(resp.source_address);
            },
            MessageVariant::EntityStatusRequestVariant(req) => {
                let response = EntityStatusResponse::new(
                    NodeType::Node,
                    self.max_sockets,
                    self.open_sockets,
                    self.max_data_size,
                );
                stream.write_all(&response.serialize()).unwrap();
            }
            MessageVariant::DiagnoticMessageVariant(msg) => todo!(),
            MessageVariant::DiagnosticPowerModeRequestVariant(req) => todo!(),
            _ => (),
        }
    }
    fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buff: Vec<u8> = Vec::<u8>::with_capacity(8);
        let mut connection_state = ConnectionState::initialized;
        loop {
            buff.resize(8, 0);
            match stream.read_exact(&mut buff) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Error during socket read, closing");
                    return;
                }
            }
            let payload_len = match DoIPHeader::from_buffer(&buff) {
                Ok(header) => header.payload_length,
                Err(code) => {
                    stream
                        .write_all(&HeaderNackMessage::new(code).serialize())
                        .unwrap();
                    if code == NackCode::IncorrectPattern || code == NackCode::InvalidPayloadLength
                    {
                        return;
                    }
                    DoIPHeader::get_payload_len(&buff)
                }
            };
            let mut payload_buff: Vec<u8> = Vec::<u8>::with_capacity(payload_len as usize);
            payload_buff.resize(payload_len as usize, 0);
            match stream.read_exact(&mut payload_buff) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("Error during socket read, closing");
                    return;
                }
            }
            buff.extend_from_slice(&payload_buff);
            match message_factory(&buff) {
                Ok(message) =>self.handle_message(stream, &message),
                Err(code) => {
                    stream
                        .write_all(&HeaderNackMessage::new(code).serialize())
                        .unwrap();
                    if code == NackCode::InvalidPayloadLength {
                        return;
                    }
                }
            }
        }
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
        let handle = thread::spawn(move || {
            DoIPServer::identification_handler(announcement_message);
        });
        let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, DoIPServer::DOIP_PORT)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(&mut stream),
                Err(_) => eprint!("Invalid stream received"),
            }
        }
        handle.join().unwrap();
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
        socket.send_to(
            &response.serialize(),
            (Ipv4Addr::BROADCAST, DoIPServer::DOIP_PORT),
        )?;
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
    fn is_id_req_addr_us(
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
    fn identification_handler(response: VehicleIdentificationResponse) -> ! {
        let mut header_buff: [u8; 40] = [0; 40];
        let socket = DoIPServer::init_udp_socket().expect("UDP socket setup failed");
        DoIPServer::announce_wait_random();
        DoIPServer::announce_on_upd_socket(&socket, &response).expect("Announcement failed");
        loop {
            if let Ok((len, addr)) = socket.recv_from(&mut header_buff) {
                match message_factory(&header_buff) {
                    Ok(message) => {
                        if DoIPServer::is_id_req_addr_us(&message, &response) {
                            if let Err(r) = socket.send_to(&response.serialize(), addr) {
                                eprintln!("Error during sending announcement: {}", r);
                            }
                        }
                    }
                    Err(code) => {
                        eprintln!("Identification message parsing failed: {:?}", code);
                    }
                }
            } else {
                eprintln!("Error during recv_from");
            }
        } // loop
    }
}

#[derive(Default)]
pub struct DoIPServerBuilder {
    server: DoIPServer,
}
impl DoIPServerBuilder {
    pub fn new() -> Self {
        let mut server: DoIPServer = Default::default();
        server.max_sockets = 10;
        server.open_sockets = 0;
        server.max_data_size = u32::max_value();
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
        let mut server_builder: DoIPServerBuilder = DoIPServerBuilder::default();
        server_builder
            .set_vin(&vin)
            .set_eid(&eid)
            .set_gid(&gid)
            .set_logical_address(logical_address);
        let server = server_builder.get_server();
    }
}
