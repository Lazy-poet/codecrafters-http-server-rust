// Uncomment this block to pass the first stage
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let port = "4221";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let content = "Hello World";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
