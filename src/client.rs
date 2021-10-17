use std::net::UdpSocket;
use std::io;
use std::env;
use std::sync::Arc;
use std::thread;
use netfunc::{send_packet,recv_packet};

struct PeerModel {
    socket_recv: UdpSocket,
    socket_send: UdpSocket,
}

impl PeerModel {
    pub fn _new(recv_addr: String, send_addr: String) -> Self{
        PeerModel {
            socket_recv: UdpSocket::bind(recv_addr).unwrap(),
            socket_send: UdpSocket::bind(send_addr).unwrap(),
        }
    }
    
    pub fn _send(&self, ext_addr:String){
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read message");
            send_packet(buffer, &self.socket_send, &ext_addr);
        }
    }
    
    pub fn _receive(&self){
        loop{
            let buffer = String::new();
            recv_packet(&buffer, &self.socket_recv);
        }
    }
}

struct Peer { model: Arc<PeerModel> }

impl Peer {
    pub fn new(recv_addr: String, send_addr: String) -> Self {
        Peer {
            model: Arc::new(PeerModel::_new(recv_addr, send_addr))
        }
    }

    pub fn start(&self, ext_addr: String){
        let local_self = self.model.clone();
        thread::spawn(move || {
            local_self._receive();
        });
        
        self.model._send(ext_addr);
    }
}


fn main() {

    // Sample Inputs:
    // Client 1: cargo run --bin client -- "127.0.0.1:8080" "127.0.0.1:8081" "127.0.0.1:8082"
    // Client 2: cargo run --bin client -- "127.0.0.1:8083" "127.0.0.1:8082" "127.0.0.1:8081"
    // Receive and External have to be matching.

    let args: Vec<String> = env::args().collect();
    println!("Send Socket Address: {:?}", args[1]);
    println!("Receive Socket Address: {:?}", args[2]);
    println!("External Host Address: {:?}", args[3]);
    
    let haddr_send = args[1].clone();
    let haddr_recv = args[2].clone();   
    let addr = args[3].clone();    
    
    let peer = Peer::new(haddr_recv, haddr_send);
    peer.start(addr);
}

