use std::net::UdpSocket;
use std::env;
use std::thread;
use std::sync::Arc;
use netfunc::{tracker_send_packet, identity_response, TrackerPacket};

struct ServerModel {
    socket: UdpSocket,
}

impl ServerModel {
    pub fn _new(host_addr: String) -> Self{
        ServerModel {
            socket: UdpSocket::bind(host_addr).unwrap()
        }
    }
    
    pub fn _send(&self, ext_addr:String){
        loop {
            // let mut buffer = String::new();
            let buffer = TrackerPacket::_new(
                String::from("monkeywings"),
                2,
                false,
                0,
                8080,
                [127, 0, 0, 1]
            );
            // io::stdin().read_line(&mut buffer).expect("Failed to read message");

            tracker_send_packet(buffer, &self.socket, &ext_addr);
        }
    }
    
    pub fn _receive(&self){
        loop{
            let buffer = String::new();
            identity_response(&buffer, &self.socket);
        }
    }
}

struct Server { model: Arc<ServerModel> }

impl Server {
    pub fn new(recv_addr: String) -> Self {
        Server {
            model: Arc::new(ServerModel::_new(recv_addr))
        }
    }

    pub fn start(&self, _ext_addr: String){
        // let local_self = self.model.clone();
        // thread::spawn(move || {
            // local_self._receive();
        // });
        self.model._receive();
        
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
    
    let peer = Server::new(host_addr);
    peer.start(ext_addr);
}

