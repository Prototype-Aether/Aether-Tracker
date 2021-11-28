use std::net::UdpSocket;
use std::io;
use std::env;
use std::sync::Arc;
use std::{thread, time};
use netfunc::{identity_report};

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

    pub fn keepalive(&self, ext_addr:String){
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read message");
        loop {
            identity_report(String::from(username.trim()), &self.socket, &ext_addr);
            thread::sleep(time::Duration::from_secs_f32(1.3));
        }
    }

    pub fn getpeer(&self, peer_id:String){ }
    
}

struct Peer { model: Arc<PeerModel> }

impl Peer {
    pub fn new(recv_addr: String) -> Self {
        Peer {
            model: Arc::new(PeerModel::_new(recv_addr))
        }
    }

    pub fn start(&self, ext_addr: String){
        let local_self = self.model.clone();
        let joinit = thread::spawn(move || {
            local_self.keepalive(ext_addr);
        });
        joinit.join();
        // self.model._send(ext_addr);
    }
}


fn main() {

    // Sample Inputs:
    // Client 1: cargo run --bin client -- "8081" "127.0.0.1:8082"

    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Error: Correct format is cargo run --bin client -- \"HostPort\" \"TrackerAddress\"");
        return;
    }
    println!("Client Host Address: {:?}", format!("0.0.0.0:{}",args[1]));
    println!("Tracker Address: {:?}", args[2]);
    
    let host_addr = format!("0.0.0.0:{}",args[1]);   
    let ext_addr = args[2].clone();    
    
    let peer = Peer::new(host_addr);
    peer.start(ext_addr);
}

