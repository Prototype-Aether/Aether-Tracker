use std::net::UdpSocket;
use netfunc::{send_packet,recv_packet};


fn main() {
    // Start the run time
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let future = sender();
    rt.block_on(future);

    let mut rt2 = tokio::runtime::Runtime::new().unwrap();
    let future2 = receiver();
    rt2.block_on(future2);
}

async fn sender(){
    let buffer = String::from("Hello, world!");
    let haddr = "127.0.0.1:8081";
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(haddr).unwrap();

    send_packet(buffer,socket,addr);
}

async fn receiver(){
    let buffer = String::new();
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).unwrap();
    loop {
        recv_packet(&buffer, &socket);
    }
}