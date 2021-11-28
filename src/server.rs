use std::net::UdpSocket;
use std::env;
use std::sync::Arc;
use netfunc::{identity_response};

struct ServerModel {
    socket: UdpSocket,
}

impl ServerModel {
    pub fn _new(host_addr: String) -> Self{
        ServerModel {
            socket: UdpSocket::bind(host_addr).unwrap()
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

    pub fn start(&self){
        self.model._receive();
    }
}


fn main() {

    // Sample Input:
    // Server: cargo run --bin server -- "127.0.0.1:8081" 

    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Error: Correct format is cargo run --bin server -- \"TrackerServerAddress\"");
        return;
    }
    println!("Client Host Address: {:?}", args[1]);
    
    let host_addr = args[1].clone();   
    
    let server = Server::new(host_addr);
    server.start();
}

