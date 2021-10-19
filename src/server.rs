use std::net::UdpSocket;
use netfunc::recv_packet;


fn main() {
    let buffer = String::new();
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).unwrap();



    loop {
        recv_packet(&buffer, &socket);
    }
}
