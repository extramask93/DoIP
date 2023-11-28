use crate::message::{
    header::{DoIPHeader, NackCode},
    message_factory,
    vehicle_identification::{FurtherAction, VehicleIdentificationResponse},
    Message, MessageVariant,
};
use rand::Rng;
use std::{
    io::{self},
    net::{TcpListener, TcpStream, UdpSocket, IpAddr, Ipv4Addr, Ipv6Addr},
    thread::{self},
    time::Duration,
};

#[derive(Default)]
pub struct DoIPClient {
}
impl DoIPClient {
    const DOIP_PORT: u16 = 13200;
    pub fn start(&self) {
        let handle = thread::spawn(move||{
            DoIPClient::identification_handler();}
            );
        handle.join().unwrap();
    }
    fn init_udp_socket() -> io::Result<UdpSocket> {
        let socket: UdpSocket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DoIPClient::DOIP_PORT))?;
        socket.set_broadcast(true)?;
        socket.set_write_timeout(Some(Duration::from_secs(5)))?;
        Ok(socket)
    }
    fn parse_identification_response(buff: &[u8], len: usize) -> Result<MessageVariant, NackCode> {
        let header = DoIPHeader::from_buffer(&buff[0..len])?;
        message_factory(&header, &buff[DoIPHeader::length()..])
    }
    fn identification_handler() {
        let mut header_buff: [u8; DoIPHeader::length() + 17] = [0; DoIPHeader::length() + 17];
        let socket = DoIPClient::init_udp_socket().expect("UDP socket setup failed");
        loop {
            if let Ok((len, _)) = socket.recv_from(&mut header_buff) {
                match DoIPClient::parse_identification_response(&header_buff, len) {
                    Ok(message) => {
                        match message {
                            MessageVariant::VehicleIDResVariant(response) => {
                                println!("{:?}", response);
                            },
                            _ => println!("Invalid message recieved on udp port")
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_client() {
    }
}
