use std::net::UdpSocket;
use std::io;
// use std::env;
use std::sync::Arc;
use std::thread;
use netfunc::{send_packet,recv_packet};

struct PeerModel {
    socket_recv: UdpSocket,
    socket_send: UdpSocket,
}

struct Peer { model: Arc<PeerModel> }

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
    
    // Read this https://stackoverflow.com/questions/67244233/wrapping-asyncread-self-has-an-anonymous-lifetime-but-it-needs-to-satisfy?rq=1
    pub fn _receive(&self){
        loop{
            let buffer = String::new();
            recv_packet(&buffer, &self.socket_recv);
        }
    }
}

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
    // let args: Vec<String> = env::args().collect();
    // println!("Send Socket Address: {:?}", args[1]);
    // println!("Receive Socket Address: {:?}", args[2]);
    // println!("External Host Address: {:?}", args[3]);
    
    // Setup 2 sockets, one for send and another for recv
    let haddr_send = String::from("127.0.0.1:8082");
    
    let haddr_recv = String::from("127.0.0.1:8081");   // Host
    let addr = String::from("127.0.0.1:8080");    // Other peer
    
    let peer = Peer::new(haddr_recv, haddr_send);
    peer.start(addr);
}

