use std::net::UdpSocket;

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
pub fn recv_packet(_buffer:&str, socket: &UdpSocket,) {
    let mut buff = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buff).expect("Not received");
    let data = String::from_utf8_lossy(&buff[..amt]);
    println!("Received {} bytes from {}", amt, src);
    println!("Data: {}", data);
}