use netfunc::{decode, encode, identity_response};
use std::collections::HashMap;
use std::env;
use std::net::UdpSocket;

struct PeerInfo {
    ip_address: [u8; 4],
    port: u16,
}

impl PeerInfo {
    pub fn new(ip_address: [u8; 4], port: u16) -> Self {
        PeerInfo { ip_address, port }
    }
}

struct TrackerServer {
    socket: UdpSocket,
    peers: HashMap<String, PeerInfo>,
}

impl TrackerServer {
    pub fn new(host_addr: String) -> Self {
        TrackerServer {
            socket: UdpSocket::bind(host_addr).unwrap(),
            peers: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
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
                    username.retain(|c| !c.is_whitespace()); // Get rid of whitspaces

                    // Check if peer is stored
                    let peer_info = self.peers.get(&username);
                    match peer_info {
                        None => {},
                        Some(peer_info) => {
                            let packet =
                                encode(username, 2, false, 1, peer_info.port, peer_info.ip_address);
                            (self.socket)
                                .send_to(packet.as_bytes(), src.to_string())
                                .expect("Not sent");
                        }
                    };
                }
            }

            // Remove later, just to view stack
            println!("\nStorage");
            for key in self.peers.keys() {
                println!("Username: {}", key);
            }
        }
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();

    let port = args[1].clone();
    let tracker_addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", port);

    let mut server = TrackerServer::new(tracker_addr);
    server.start();
}
