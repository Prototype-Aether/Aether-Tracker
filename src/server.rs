use aether_lib::tracker::TrackerPacket;
use netfunc::{identity_response, PeerInfo};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::net::UdpSocket;

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
            let data: TrackerPacket = TryFrom::try_from(buffer[..amt].to_vec()).unwrap();

            if data.req {
                // Reply to Identity Request
                if data.packet_type == 0 {
                    let (mut username, identity_number, ip, port) = identity_response(data, src, &self.socket);
                    username.retain(|c| !c.is_whitespace());
                    println!("Username to be added: {}", username);
                    let key = format!("{}{}", username, identity_number);
                    self.peers.insert(key, PeerInfo::new(ip, port));
                } else if data.packet_type == 2 {
                    println!("{} is polling.", data.username);
                }
                // Reply to Connection Request
                else if data.packet_type == 1 {
                    println!("Received a request for {}", data.peer_username);
                    let mut username = data.peer_username;
                    username.retain(|c| !c.is_whitespace()); // Get rid of whitspaces

                    let key = format!("{}{}",username, data.identity_number);

                    // Check if peer is stored
                    let peer_info = self.peers.get(&key);
                    match peer_info {
                        None => {}
                        Some(peer_info) => {
                            let packet: TrackerPacket = TrackerPacket {
                                identity_number: 2,
                                peer_username: "".to_string(),
                                connections: Vec::new(),
                                username: username,
                                req: false,
                                packet_type: 1 as u8,
                                port: peer_info.port,
                                ip: peer_info.ip_address,
                            };
                            let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
                            (self.socket)
                                .send_to(&buffer, src.to_string())
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
