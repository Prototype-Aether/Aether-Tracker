use aether_lib::tracker::TrackerPacket;
use itertools::Itertools;
use netfunc::{connection_poll, identity_report, identity_request, connection_request, PeerInfo};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::io;
use std::net::UdpSocket;
use std::sync::Arc;
use std::{thread, time};

struct PeerModel {
    socket: UdpSocket,
    username: String,
    peer: HashMap<String, PeerInfo>,
    requests: Vec<String>,
    // [0] => identity = 1 => peer = B
    // [1] => identity = 2 => peer = C
}

impl PeerModel {
    pub fn _new(username: String, host_addr: String) -> Self {
        PeerModel {
            socket: UdpSocket::bind(host_addr).unwrap(),
            username: username,
            peer: HashMap::new(),
            requests: Vec::new(),
        }
    }

    pub fn keepalive(&self, username: String, tracker_addr: String, identity_number: u32) {
        loop {
            connection_poll(
                username.clone(),
                identity_number, // Not necessary
                &self.socket,
                tracker_addr.clone(),
            );
            thread::sleep(time::Duration::from_secs_f32(2.0));
        }
    }

    pub fn listener(&mut self, tracker_addr: String) {
        loop {
            let mut buffer = [0; 2048];
            let (amt, _src) = self.socket.recv_from(&mut buffer).unwrap();
            let data: TrackerPacket = TryFrom::try_from(buffer[..amt].to_vec()).unwrap();

            if !data.req {
                match Option::from(data.packet_type) {
                    // Process Response to Identity Request
                    Some(1) => {
                        // Establish connection here
                        let addr = format!("{}:{}", data.ip.iter().join("."), data.port);
                        println!("Sending to {}", addr);
                        let peer_username = format!("Hey There {}!", data.username);

                        let packet: TrackerPacket = TrackerPacket {
                            identity_number: 2,
                            peer_username: peer_username.clone(),
                            connections: Vec::new(),
                            username: peer_username.clone(),
                            req: false,
                            packet_type: 5 as u8,
                            port: data.port,
                            ip: data.ip,
                        };
                        let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();

                        (self.socket).send_to(&buffer, addr).expect("Not sent");
                    }

                    // Process response to Connection Poll
                    Some(3) => {
                        if !data.connections.is_empty() {
                            for connection in data.connections.iter() {
                                // match self.requests.iter().position(|&user| user == data.username))
                                // If A has already sent a connection Request
                                if self.requests.contains(&connection.username)
                                {
                                    // Handshake occurs from here
                                    let index: u32 = self.requests.iter().position(|&user| user == data.username).unwrap() as u32 + 1;
                                    let packet: TrackerPacket = TrackerPacket {
                                        identity_number: index,
                                        peer_username: "".to_string(),
                                        connections: Vec::new(),
                                        username: "Hey we did it!".to_string(),
                                        req: false,
                                        packet_type: 5 as u8,
                                        port: 0000,
                                        ip: [0, 0, 0, 0],
                                    };
                                    let buffer: Vec<u8> = TryFrom::try_from(packet).unwrap();
                                    let addr = format!(
                                        "{}:{}",
                                        connection.ip.iter().join("."),
                                        connection.port
                                    );
                                    (self.socket).send_to(&buffer, addr).expect("Not sent");
                                    
                                }

                                // A spawns a connection request
                                else {
                                    self.requests.push(connection.username.clone());
                                    connection_request(self.username.clone(), data.identity_number + 1, connection.username.clone(), &self.socket, tracker_addr.clone());
                                }
                            }
                        }
                    }

                    // Just a test
                    Some(5) => {
                        println!("{}", data.username);
                    }
                    _ => {}
                }
            }
        }
    }
}

struct Peer {
    model: Arc<PeerModel>,
}

impl Peer {
    pub fn new(username:String, recv_addr: String) -> Self {
        Peer {
            model: Arc::new(PeerModel::_new(username, recv_addr)),
        }
    }

    pub fn start(&self, tracker_addr: String) {
        // Report identity to tracker
        

        let mut buffer = [0; 2048];
        self.model
            .socket
            .set_read_timeout(Some(time::Duration::from_secs_f32(1.0)))
            .expect("set_read_timeout call failed");

        // Loop until reply is received.
        loop {
            identity_report(
                String::from(self.model.username.trim()),
                &self.model.socket,
                &tracker_addr,
                false,
            );
            let tracker_response = self.model.socket.recv_from(&mut buffer);
            match tracker_response {
                Ok(_) => break,
                Err(_error) => {
                    println!("No response, running again");
                }
            }
        }
        self.model
            .socket
            .set_read_timeout(None)
            .expect("set_read_timeout call failed");

        // let mut peer = String::new();
        // io::stdin()
        //     .read_line(&mut peer)
        //     .expect("Failed to read message");
        // identity_request(peer, &self.model.socket, tracker_addr.clone());

        // let tracker_addr_copy = String::from(tracker_addr.clone());

        // let keepalive_thread = self.model.clone();
        let listener_thread = self.model.clone();
        // thread::spawn(move || {
        //     keepalive_thread.keepalive(username, tracker_addr_copy);
        // });

        thread::spawn(move || {
            listener_thread.listener(tracker_addr);
        });
        let mut peer = String::new();
        println!("Enter peer to connect: ");
        io::stdin()
            .read_line(&mut peer)
            .expect("Failed to read message");

        loop {
            identity_request(peer.clone(), &self.model.socket, tracker_addr.clone());
            thread::sleep(time::Duration::from_secs_f32(2.5));
        }
    }
}

fn main() {
    // Sample Inputs:
    // Client 1: cargo run --bin client -- "8081" "149.129.129.226:8982"

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Error: Correct format is cargo run --bin client -- \"HostPort\" \"TrackerAddress\""
        );
        return;
    }
    println!("Client Host Address: {:?}", format!("0.0.0.0:{}", args[1]));
    println!("Tracker Address: {:?}", args[2]);

    let host_addr = format!("0.0.0.0:{}", args[1]);
    let tracker_addr = args[2].clone();
    println!("Enter username: ");
    let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read message");
    let peer = Peer::new(username, host_addr);
    peer.start(tracker_addr);
}
