// node.rs
// SynkNodes – WarpSynk Protocol
// Example Rust structure for node logic

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

struct SynkNode {
    id: String,
    port: u16,
}

impl SynkNode {
    fn new(id: &str, port: u16) -> Self {
        SynkNode {
            id: id.to_string(),
            port,
        }
    }

    fn start(&self) {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&address)
            .expect("Failed to bind port");

        println!("SynkNode {} running on {}", self.id, address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let node_id = self.id.clone();
                    thread::spawn(move || {
                        handle_client(stream, node_id);
                    });
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, node_id: String) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            println!("Node {} received: {}", node_id, request);

            let response = format!("SynkNode {} ACK\n", node_id);
            stream.write(response.as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Error reading stream: {}", e),
    }
}

fn main() {
    let node = SynkNode::new("synk-001", 8080);
    node.start();
}