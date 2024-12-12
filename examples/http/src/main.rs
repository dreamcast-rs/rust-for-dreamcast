use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

kos::INIT_FLAGS!(INIT_DEFAULT | INIT_NET);

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<!DOCTYPE html><html><body><h1>Hello, World!</h1></body></html>";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    println!("Hello, world from Rust! - http example");

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening at {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
