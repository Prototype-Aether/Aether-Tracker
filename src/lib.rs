use aether_lib::tracker::TrackerPacket;
use std::convert::TryFrom;
use std::net::{IpAddr, SocketAddr, UdpSocket};

pub struct PeerInfo {
    pub ip_address: [u8; 4],
    pub port: u16,
}

impl PeerInfo {
    pub fn new(ip_address: [u8; 4], port: u16) -> Self {
        PeerInfo { ip_address, port }
    }
}

pub fn identity_report(identity_username: String, socket: &UdpSocket, addr: &str, _verbose: bool) {
    let packet: TrackerPacket = TrackerPacket {
        identity_number: 2,
        peer_username: "".to_string(),
        connections: Vec::new(),
        username: identity_username,
        req: true,
        packet_type: 0 as u8,
        port: 8080,
        ip: [0, 0, 0, 0],
    };
    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
    (*socket).send_to(&buffer, addr).expect("Not sent");
}

pub fn identity_confirm(
    data: TrackerPacket,
    src: SocketAddr,
    socket: &UdpSocket,
) -> (String, u32, [u8; 4], u16) {
    // Process Request and send Response
    let addr = src.to_string();
    let ip_bytes = match src.ip() {
        IpAddr::V4(ip) => ip.octets(),
        IpAddr::V6(_ip) => unreachable!(),
    };
    let port = src.port();
    let packet: TrackerPacket = TrackerPacket {
        identity_number: 2,
        peer_username: "".to_string(),
        connections: Vec::new(),
        username: data.username.clone(),
        req: false,
        packet_type: 0 as u8,
        port: port,
        ip: ip_bytes,
    };
    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
    (*socket).send_to(&buffer, addr.clone()).expect("Not sent");

    return (
        data.username.trim().to_string(),
        data.identity_number,
        ip_bytes,
        port,
    );
}

pub fn identity_request(peer_username: String, socket: &UdpSocket, tracker_addr: String) {
    let packet: TrackerPacket = TrackerPacket {
        identity_number: 2,
        peer_username: peer_username.trim().to_string(),
        connections: Vec::new(),
        username: "".to_string(),
        req: true,
        packet_type: 1 as u8,
        port: 8080,
        ip: [0, 0, 0, 0],
    };
    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
    (*socket).send_to(&buffer, tracker_addr).expect("Not sent");
}

pub fn connection_poll(username: String, id: u32, socket: &UdpSocket, tracker_addr: String) {
    let packet: TrackerPacket = TrackerPacket {
        identity_number: id,
        peer_username: "".to_string(),
        connections: Vec::new(),
        username: username,
        req: true,
        packet_type: 3 as u8,
        port: 8080,
        ip: [0, 0, 0, 0],
    };
    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
    (*socket).send_to(&buffer, tracker_addr).expect("Not sent");
}

pub fn connection_request(username: String, identity_number: u32, peer_username: String, socket: &UdpSocket, tracker_addr: String) {
    let packet: TrackerPacket = TrackerPacket {
        identity_number: identity_number,
        peer_username: peer_username,
        connections: Vec::new(),
        username: username,
        req: true,
        packet_type: 2 as u8,
        port: 8080,
        ip: [0, 0, 0, 0],
    };
    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
    (*socket).send_to(&buffer, tracker_addr).expect("Not sent");
}
