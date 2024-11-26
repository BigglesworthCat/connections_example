use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(size) => {
                let received = String::from_utf8_lossy(&buffer[0..size]);
                println!("Received: {}", received);
                stream
                    .write_all(format!("Server received: {}", received).as_bytes())
                    .unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
    println!("Connection closed.");
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind address");
    println!("Server running on 0.0.0.0:8888");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
