use std::net::UdpSocket;
use std::io;
use netfunc::{send_packet,recv_packet};

// struct Peer {
//     socket: UdpSocket,
// }

// impl Peer {
//     pub fn new(address: String) -> Self{
//         Peer {
//             socket: UdpSocket::bind(address).unwrap(),
//         }
//     }
    
//     pub fn send(&self, address:String){
//         let buffer = String::from("Helo there");
//         // io::stdin().read_line(&mut buffer).expect("Failed to read message");
//         send_packet(buffer, &self.socket, &address);
//     }

//     // Read this https://stackoverflow.com/questions/67244233/wrapping-asyncread-self-has-an-anonymous-lifetime-but-it-needs-to-satisfy?rq=1
//     pub fn receive(&self){
//         std::thread::spawn(move || {
//             loop{
//                 let buffer = String::new();
//                 recv_packet(&buffer, &self.socket);
//             }
//         });
//     }
// }


fn main() {
    // Setup 2 sockets, one for send and another for recv
    let haddr_send = "127.0.0.1:8083";

    let haddr_recv = "127.0.0.1:8080";   // Host
    let addr = "127.0.0.1:8081";    // Other peer
    
    let socket_recv = UdpSocket::bind(haddr_recv).unwrap();
    let socket_send = UdpSocket::bind(haddr_send).unwrap();
    
    std::thread::spawn(move || {
        receiver(&socket_recv);
    });
    loop {
        sender(&socket_send, &addr);
    }
}

fn sender(socket: &UdpSocket, addr: &str){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read message");
    send_packet(buffer,socket,addr);
}

fn receiver(socket: &UdpSocket,){
    let buffer = String::new();
    loop {
        recv_packet(&buffer, &socket);
    }
}