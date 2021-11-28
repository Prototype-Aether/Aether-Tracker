use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::net::{UdpSocket, IpAddr};

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

pub fn encode(username: String, id: u32, request: bool, p_type: u8, port_no: u16, ip: [u8; 4]) -> String{
    let packet = TrackerPacket::_new(username, id, request, p_type, port_no, ip);
    let buffer = serde_json::to_string(&packet).unwrap();
    return buffer;
}

pub fn decode(buffer: [u8; 2048], amt: usize) -> TrackerPacket{
    let data_string = String::from_utf8_lossy(&buffer[..amt]);
    let data: TrackerPacket = serde_json::from_str(&data_string).unwrap();
    return data;
}


pub fn identity_report(username: String, socket: &UdpSocket, addr: &str) -> TrackerPacket {

    // Send the Request 
    let buffer = encode(username, 2, true, 0, 8080, [0, 0, 0, 0]);
    (*socket)
        .send_to(&buffer.as_bytes(), addr)
        .expect("Not sent");

    // Process Reponse
    let mut buffer = [0; 2048];
    let (amt, src) = socket.recv_from(&mut buffer).expect("Not received");
    let data = decode(buffer, amt);

    println!("Received {} bytes from {}", amt, src);
    println!(
        "Username: {}\nIP: {}\nPort: {}",
        data.username,
        data.ip.iter().join("."),
        data.port
    );
    return data;
}

pub fn identity_response(_buffer: &str, socket: &UdpSocket) -> (String, [u8; 4], u16) {

    // Receive Report Request
    let mut buffer = [0; 2048];
    let (amt, src) = socket.recv_from(&mut buffer).expect("Not received");
    let data = decode(buffer, amt);
    println!("Received {} bytes from {}", amt, src);
    println!("Username P: {}", data.username);


    // Process Request and send Response
    let addr = src.to_string();
    let ip_bytes = match src.ip() {
        IpAddr::V4(ip) => ip.octets(),
        IpAddr::V6(_ip) => unreachable!()
    };
    let port = src.port();
    let buffer = encode(data.username.clone(), 2, false, 0, port, ip_bytes);
    (*socket)
        .send_to(&buffer.as_bytes(), addr.clone())
        .expect("Not sent");

    // Return username, ip and port
    return (data.username, ip_bytes, port);
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