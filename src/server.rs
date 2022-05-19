use aether_lib::tracker::{ConnectionRequest, TrackerPacket};
use netfunc::{identity_confirm, PeerInfo};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::net::{IpAddr, UdpSocket};

struct TrackerServer {
    socket: UdpSocket,
    peers: HashMap<String, PeerInfo>,
    requests: HashMap<String, Vec<ConnectionRequest>>,
}

impl TrackerServer {
    pub fn new(host_addr: String) -> Self {
        TrackerServer {
            socket: UdpSocket::bind(host_addr).unwrap(),
            peers: HashMap::new(),
            requests: HashMap::new(),
        }
    }

    pub fn store_peer(&mut self, username: String, identity_number: u32, ip: [u8; 4], port: u16) {
        println!("Username to be added: {}", username);
        let key = format!("{}{}", username, identity_number);
        self.peers.insert(key, PeerInfo::new(ip, port));
    }
    // pub fn check_and_add(&mut self, request_list )
    pub fn update(
        request_list: &mut Vec<ConnectionRequest>,
        new_request: ConnectionRequest,
    ) -> Option<ConnectionRequest> {
        for request in request_list {
            if (*request).username == new_request.username
                && (*request).identity_number == new_request.identity_number
            {
                (*request).ip = new_request.ip;
                (*request).port = new_request.port;
                return None;
            }
        }
        Some(new_request)
    }
    pub fn start(&mut self) {
        loop {
            // Receive Report Request
            let mut buffer = [0; 2048];
            let (amt, src) = self.socket.recv_from(&mut buffer).expect("Not received");
            let data: TrackerPacket = TryFrom::try_from(buffer[..amt].to_vec()).unwrap();

            if data.req {
                match Option::from(data.packet_type) {
                    // Process Identity Report
                    Some(0) => {
                        let (username, identity_number, ip, port) =
                            identity_confirm(data, src, &self.socket);
                        self.store_peer(username, identity_number, ip, port);
                    }

                    // Process Identity Request
                    Some(1) => {
                        println!("Received a request for {}", data.peer_username);

                        let key = format!("{}{}", data.peer_username, data.identity_number);

                        // Check if peer is stored
                        let peer_info = self.peers.get(&key);
                        match peer_info {
                            None => {}
                            Some(peer_info) => {
                                let packet: TrackerPacket = TrackerPacket {
                                    identity_number: 2,
                                    peer_username: "".to_string(),
                                    connections: Vec::new(),
                                    username: data.peer_username,
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
                    Some(2) => {
                        let x = data.peer_username.clone();
                        println!("Received a request for {}", &x[(x.len() - 8)..]);
                        let ip_bytes = match src.ip() {
                            IpAddr::V4(ip) => ip.octets(),
                            IpAddr::V6(_ip) => unreachable!(),
                        };
                        let connection: ConnectionRequest = ConnectionRequest {
                            identity_number: data.identity_number,
                            username: data.username,
                            port: src.port(),
                            ip: ip_bytes,
                        };

                        let mut requests_list = self
                            .requests
                            .entry(data.peer_username)
                            .or_insert(Vec::new());

                        match TrackerServer::update(&mut requests_list, connection) {
                            Some(conn) => requests_list.push(conn),
                            None => (),
                        }
                    }

                    Some(3) => {
                        let x = data.username.clone();
                        println!("Poll from {}", &x[(x.len() - 8)..]);
                        let connection_list = self.requests.get(&data.username);
                        let connection_list: Vec<ConnectionRequest> = match connection_list {
                            None => Vec::new(),
                            Some(conn_list) => conn_list.clone(),
                        };

                        let packet: TrackerPacket = TrackerPacket {
                            identity_number: 2,
                            peer_username: "".to_string(),
                            connections: connection_list.clone(),
                            username: data.peer_username,
                            req: false,
                            packet_type: 3 as u8,
                            port: 0000,
                            ip: [0, 0, 0, 0],
                        };
                        let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
                        (self.socket)
                            .send_to(&buffer, src.to_string())
                            .expect("Not sent");
                    }
                    _ => {}
                }
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
