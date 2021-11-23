use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::net::{UdpSocket, IpAddr};
/// Sends a packet to another peer.
///
/// ### Arguments
///
/// * `buffer` - Message or data to be sent.
/// * `socket`- Socket that is bound to the sender.
/// * `addr` - The address of the other peer.
pub fn send_packet(buffer: String, socket: &UdpSocket, addr: &str) {
    let buff = buffer.as_bytes();
    (*socket).send_to(buff, addr).expect("Not sent");
}

/// Receives a packet from another peer.
///
/// ### Arguments
///
/// * `socket`- Socket that is bound to the receiving peer.
pub fn recv_packet(_buffer: &str, socket: &UdpSocket) {
    let mut buff = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data = String::from_utf8_lossy(&buff[..amt]);
    println!("Received {} bytes from {}", amt, src);
    println!("Data: {}", data);
}

#[derive(Serialize, Deserialize)]
pub struct TrackerPacket {
    username: String,
    id_num: u32,
    req: bool,
    packet_type: u8,
    port: u16,
    ip: [u8; 4],
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

pub fn tracker_send_packet(buffer: TrackerPacket, socket: &UdpSocket, addr: &str) {
    let buff = serde_json::to_string(&buffer).unwrap();
    (*socket).send_to(&buff.as_bytes(), addr).expect("Not sent");
}

pub fn tracker_recv_packet(_buffer: &str, socket: &UdpSocket) {
    let mut buff = [0; 2048];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data_string = String::from_utf8_lossy(&buff[..amt]);
    let data: TrackerPacket = serde_json::from_str(&data_string).unwrap();
    println!("Received {} bytes from {}", amt, src);
    println!("Data: {}", data.id_num);
}

pub fn identity_report(username: String, socket: &UdpSocket, addr: &str) -> TrackerPacket {

    // Send the Request 
    let packet = TrackerPacket::_new(String::from(username), 2, true, 0, 8080, [0, 0, 0, 0]);
    let buffer = serde_json::to_string(&packet).unwrap();
    (*socket)
        .send_to(&buffer.as_bytes(), addr)
        .expect("Not sent");

    // Wait for Tracker server to reply - Implement at main
    let mut buff = [0; 2048];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data_string = String::from_utf8_lossy(&buff[..amt]);
    let data: TrackerPacket = serde_json::from_str(&data_string).unwrap();

    println!("Received {} bytes from {}", amt, src);
    println!(
        "Username: {}\nIP: {}\nPort: {}",
        data.username,
        data.ip.iter().join("."),
        data.port
    );
    return data;
}

pub fn identity_response(_buffer: &str, socket: &UdpSocket) {

    // Response to the identity report - Adjust to the main
    let mut buff = [0; 2048];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data_string = String::from_utf8_lossy(&buff[..amt]);
    let data: TrackerPacket = serde_json::from_str(&data_string).unwrap();
    println!("Received {} bytes from {}", amt, src);
    println!("Data: {}", data.username);


    //
    let addr = src.to_string();
    let ip_bytes = match src.ip() {
        IpAddr::V4(ip) => ip.octets(),
        IpAddr::V6(_ip) => unreachable!()
    };
    let port = src.port();
    let packet = TrackerPacket::_new(String::from(data.username), 2, false, 0, port, ip_bytes);
    let buffer = serde_json::to_string(&packet).unwrap();
    (*socket)
        .send_to(&buffer.as_bytes(), addr)
        .expect("Not sent");
}

// pub fn connection_request(username: String, socket: &UdpSocket, addr: &str) -> TrackerPacket {

//     // Send the Request 
//     let packet = TrackerPacket::_new(String::from(username), 2, true, 0, 8080, [0, 0, 0, 0]);
//     let buffer = serde_json::to_string(&packet).unwrap();
//     (*socket)
//         .send_to(&buffer.as_bytes(), addr)
//         .expect("Not sent");

//     // Wait for Tracker server to reply - Implement at main
//     let mut buff = [0; 2048];
//     let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
//     let data_string = String::from_utf8_lossy(&buff[..amt]);
//     let data: TrackerPacket = serde_json::from_str(&data_string).unwrap();

//     println!("Received {} bytes from {}", amt, src);
//     println!(
//         "Username: {}\nIP: {}\nPort: {}",
//         data.username,
//         data.ip.iter().join("."),
//         data.port
//     );
//     return data;
// }