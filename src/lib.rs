use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr, UdpSocket};

#[derive(Serialize, Deserialize)]
pub struct TrackerPacket {
    pub username: String,
    id_num: u32,
    pub req: bool,
    pub packet_type: u8,
    pub port: u16,
    pub ip: [u8; 4],
}

impl TrackerPacket {
    pub fn _new(
        uname: String,
        id: u32,
        request: bool,
        p_type: u8,
        port_no: u16,
        ip: [u8; 4],
    ) -> Self {
        TrackerPacket {
            username: uname,
            id_num: id,
            req: request,
            packet_type: p_type,
            port: port_no,
            ip: ip,
        }
    }
}

pub fn encode(
    username: String,
    id: u32,
    request: bool,
    p_type: u8,
    port_no: u16,
    ip: [u8; 4],
) -> String {
    let packet = TrackerPacket::_new(username, id, request, p_type, port_no, ip);
    let buffer = serde_json::to_string(&packet).unwrap();
    return buffer;
}

pub fn decode(buffer: [u8; 2048], amt: usize) -> TrackerPacket {
    let data_string = String::from_utf8_lossy(&buffer[..amt]);
    let data: serde_json::Result<TrackerPacket> = serde_json::from_str(&data_string);
    let data = match data {
        Ok(data_item) => data_item,
        Err(error) => {
            // Just for testing, remove message later
            let message = String::from_utf8_lossy(&buffer[..amt]);
            panic!(
                "Error: Could not decode the packet. {}\nMessage Provided:{}",
                error, message
            )
        }
    };
    return data;
}

pub fn identity_report(username: String, socket: &UdpSocket, addr: &str, _verbose: bool) {
    let buffer = encode(username, 2, true, 0, 8080, [0, 0, 0, 0]);
    (*socket)
        .send_to(&buffer.as_bytes(), addr)
        .expect("Not sent");
}

pub fn identity_response(
    data: TrackerPacket,
    src: SocketAddr,
    socket: &UdpSocket,
    
) -> (String, [u8; 4], u16) {
    println!("{} at {}", src, data.username);

    // Process Request and send Response
    let addr = src.to_string();
    let ip_bytes = match src.ip() {
        IpAddr::V4(ip) => ip.octets(),
        IpAddr::V6(_ip) => unreachable!(),
    };
    let port = src.port();
    let buffer = encode(data.username.clone(), 2, false, 0, port, ip_bytes);
    (*socket)
        .send_to(&buffer.as_bytes(), addr.clone())
        .expect("Not sent");

    // Return username, ip and port
    return (data.username, ip_bytes, port);
}

pub fn connection_request(username: String, socket: &UdpSocket, tracker_addr: String) {
    // Send the Request
    let buffer = encode(username, 2, true, 1, 8080, [0, 0, 0, 0]);
    (*socket)
        .send_to(&buffer.as_bytes(), tracker_addr)
        .expect("Not sent");
}