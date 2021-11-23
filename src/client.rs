use std::net::UdpSocket;
use std::io;
use std::env;
use std::sync::Arc;
use std::thread;
use netfunc::{identity_report,tracker_recv_packet};

struct PeerModel {
    socket: UdpSocket,
}

impl PeerModel {
    pub fn _new(host_addr: String) -> Self{
        PeerModel {
            socket: UdpSocket::bind(host_addr).unwrap()
        }
    }
    
    pub fn _send(&self, ext_addr:String){
        loop {
            let mut username = String::new();
            io::stdin().read_line(&mut username).expect("Failed to read message");
            identity_report(String::from(username.trim()), &self.socket, &ext_addr);
        }
    }
    
    pub fn _receive(&self){
        loop{
            let buffer = String::new();
            tracker_recv_packet(&buffer, &self.socket);
        }
    }
}

struct Peer { model: Arc<PeerModel> }

impl Peer {
    pub fn new(recv_addr: String) -> Self {
        Peer {
            model: Arc::new(PeerModel::_new(recv_addr))
        }
    }

    pub fn start(&self, ext_addr: String){
        // let local_self = self.model.clone();
        // thread::spawn(move || {
            // local_self._send(ext_addr);
        // });
        
        self.model._send(ext_addr);
    }
}


fn main() {

    // Sample Inputs:
    // Client 1: cargo run --bin client -- "127.0.0.1:8081" "127.0.0.1:8082"
    // Client 2: cargo run --bin client -- "127.0.0.1:8082" "127.0.0.1:8081"

    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Error: Correct format is cargo run --bin client -- \"HostAddress\" \"ExternalHostAddress\"");
        return;
    }
    println!("Client Host Address: {:?}", args[1]);
    println!("External Host Address: {:?}", args[2]);
    
    let host_addr = args[1].clone();   
    let ext_addr = args[2].clone();    
    
    let peer = Peer::new(host_addr);
    peer.start(ext_addr);
}

