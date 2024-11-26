use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let ip_addr = std::env::var("IP_ADDR").unwrap();
    let port = std::env::var("PORT").unwrap_or("8888".to_owned());

    let url = format!("{ip_addr}:{port}");
    println!("Using {url} for connection");

    let mut stream = TcpStream::connect(url).expect("Failed to connect to server");
    println!("Connected to the server!");

    let mut input = String::new();
    loop {
        input.clear();
        println!("Enter a message to send (or type 'exit' to quit): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let message = input.trim();
        if message.eq_ignore_ascii_case("exit") {
            break;
        }

        stream
            .write_all(message.as_bytes())
            .expect("Failed to send message");
        let mut buffer = [0; 1024];
        let size = stream
            .read(&mut buffer)
            .expect("Failed to receive response");
        println!(
            "Server response: {}",
            String::from_utf8_lossy(&buffer[..size])
        );
    }
}
