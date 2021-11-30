use itertools::Itertools;
use netfunc::{connection_request, decode, encode, identity_report};
use std::env;
use std::io;
use std::net::UdpSocket;
use std::sync::Arc;
use std::{thread, time};

struct PeerModel {
    socket: UdpSocket,
}

impl PeerModel {
    pub fn _new(host_addr: String) -> Self {
        PeerModel {
            socket: UdpSocket::bind(host_addr).unwrap(),
        }
    }

    pub fn _send(&self, ext_addr: String) {
        loop {
            let mut username = String::new();
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read message");
            identity_report(
                String::from(username.trim()),
                &self.socket,
                &ext_addr,
                false,
            );
        }
    }

    pub fn keepalive(&self, username: String, ext_addr: String) {
        loop {
            identity_report(
                String::from(username.trim()),
                &self.socket,
                &ext_addr,
                false,
            );
            thread::sleep(time::Duration::from_secs_f32(1.3));
        }
    }

    pub fn listener(&self, keepalive_verbose: bool) {
        loop {
            let mut buffer = [0; 2048];
            let (amt, src) = self.socket.recv_from(&mut buffer).expect("Not received");
            // let data = String::from_utf8_lossy(&buffer[..amt]);
            let data = decode(buffer, amt);
            if !data.req {
                // I tried using match, but for some reason it doesn't work well with u8
                // For Identity Response
                if data.packet_type == 0 {
                    if keepalive_verbose {
                        println!("Received {} bytes from {}", amt, src);
                        println!(
                            "Username: {}\nIP: {}\nPort: {}",
                            data.username,
                            data.ip.iter().join("."),
                            data.port
                        );
                    }
                }

                // For Connection Response
                if data.packet_type == 1 {
                    // Establish connection here
                    let addr = format!("{}:{}", data.ip.iter().join("."), data.port);
                    println!("Sending to {}", addr);
                    // println!("Received {}", addr);
                    let peer_username = format!("Hey There {}!", data.username);
                    let buffer =
                        encode(String::from(peer_username), 2, false, 2, data.port, data.ip);
                    (self.socket)
                        .send_to(&buffer.as_bytes(), addr)
                        .expect("Not sent");
                }

                // Just for testing UDP Hole Punching, using username as message
                if data.packet_type == 2 {
                    println!("{}", data.username);
                }
            }
        }
    }
}

struct Peer {
    model: Arc<PeerModel>,
}

impl Peer {
    pub fn new(recv_addr: String) -> Self {
        Peer {
            model: Arc::new(PeerModel::_new(recv_addr)),
        }
    }

    pub fn start(&self, ext_addr: String) {
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read message");
        let ext_addr2 = String::from(ext_addr.clone());

        let keepalive_thread = self.model.clone();
        let listener_thread = self.model.clone();
        thread::spawn(move || {
            keepalive_thread.keepalive(username, ext_addr2.clone());
        });

        thread::spawn(move || {
            listener_thread.listener(false);
        });
        // joinit.join();
        // self.model._send(ext_addr);
        let mut peer = String::new();
        println!("Enter peer to connect: ");
        io::stdin()
            .read_line(&mut peer)
            .expect("Failed to read message");

        loop {
            connection_request(peer.clone(), &self.model.socket, ext_addr.clone());
            thread::sleep(time::Duration::from_secs_f32(2.5));
        }
        // listener_thread_join.join();
    }
}

fn main() {
    // Sample Inputs:
    // Client 1: cargo run --bin client -- "8081" "127.0.0.1:8082"

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
    let ext_addr = args[2].clone();
    let peer = Peer::new(host_addr);
    peer.start(ext_addr);
}
