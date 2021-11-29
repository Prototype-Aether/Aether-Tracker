use netfunc::{decode, encode, identity_response};
use std::collections::HashMap;
use std::env;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

struct PeerInfo {
    ip_address: [u8; 4],
    port: u16,
}

impl PeerInfo {
    pub fn new(ip_address: [u8; 4], port: u16) -> Self {
        PeerInfo { ip_address, port }
    }
}

struct ServerModel {
    socket: UdpSocket,
    peers: HashMap<String, PeerInfo>,
}

impl ServerModel {
    pub fn _new(host_addr: String) -> Self {
        ServerModel {
            socket: UdpSocket::bind(host_addr).unwrap(),
            peers: HashMap::new(),
        }
    }

    pub fn _receive(&mut self) {
        loop {
            // Receive Report Request
            let mut buffer = [0; 2048];
            let (amt, src) = self.socket.recv_from(&mut buffer).expect("Not received");
            let data = decode(buffer, amt);

            if data.req {
                // Reply to Identity Request
                if data.packet_type == 0 {
                    let (mut username, ip, port) = identity_response(data, src, &self.socket);
                    username.retain(|c| !c.is_whitespace());
                    self.peers.insert(username, PeerInfo::new(ip, port));
                }
                // Reply to Connection Request
                else if data.packet_type == 1 {
                    println!("Received a request for {}", data.username);
                    let mut username = data.username;
                    username.retain(|c| !c.is_whitespace());

                    if self.peers.contains_key(&username) {
                        let peer_info: &PeerInfo = &self.peers[&username];
                        let packet =
                            encode(username, 2, false, 1, peer_info.port, peer_info.ip_address);
                        (self.socket)
                            .send_to(packet.as_bytes(), src.to_string())
                            .expect("Not sent");
                    } else {
                        for key in self.peers.keys() {
                            println!("{}", key);
                        }
                    }
                }
            }

            println!("\nStorage");
            for key in self.peers.keys() {
                println!("Username: {}", key);
            }
        }
    }
}

struct Server {
    model: Arc<Mutex<ServerModel>>,
}

impl Server {
    pub fn new(recv_addr: String) -> Self {
        Server {
            model: Arc::new(Mutex::new(ServerModel::_new(recv_addr))),
        }
    }

    pub fn start(&self) {
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
