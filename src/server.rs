use std::net::UdpSocket;
use std::env;
use std::sync::{Arc, Mutex};
use netfunc::{identity_response};
use std::collections::HashMap;

struct PeerInfo {
    ip_address: [u8; 4],
    port: u16
}

impl PeerInfo {
    pub fn new(ip_address: [u8; 4], port: u16) -> Self{
        PeerInfo { ip_address, port }
    }
}

struct ServerModel {
    socket: UdpSocket,
    peers: HashMap<String, PeerInfo>
}

impl ServerModel {
    pub fn _new(host_addr: String) -> Self{
        ServerModel {
            socket: UdpSocket::bind(host_addr).unwrap(),
            peers: HashMap::new()
        }
    }
    
    pub fn _receive(&mut self){
        loop{
            let buffer = String::new();
            let (username, ip, port) = identity_response(&buffer, &self.socket);
            
            self.peers.insert(username, PeerInfo::new(ip, port));

            println!("\nStorage");
            for key in self.peers.keys(){
                println!("Username: {}", key);
            }
        }
    }
}

struct Server { model: Arc<Mutex<ServerModel>> }

impl Server {
    pub fn new(recv_addr: String) -> Self {
        Server {
            model: Arc::new(Mutex::new(ServerModel::_new(recv_addr)))
        }
    }

    pub fn start(&self){
        let mut server = self.model.lock().unwrap();
        (*server)._receive();
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    let port = args[1].clone();
    let tracker_addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", port);

    let server = Server::new(tracker_addr);
    server.start();

}

