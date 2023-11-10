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

    let ok_response = "HTTP/1.1 200 OK\r\n\r\nHELLO WORLD";
    let err_response = "HTTP/1.1 404 NOT FOUND\r\n\r\nNOT FOUND";
    match get_path(buffer).as_str() {
        "/" => stream.write(ok_response.as_bytes()).unwrap(),
        _ => stream.write(err_response.as_bytes()).unwrap(),
    };

    stream.flush().unwrap();
}

fn get_path(buffer: [u8; 1024]) -> String {
    let buf_to_string = String::from_utf8_lossy(&buffer);
    let str_vec: Vec<&str> = buf_to_string.split("\r\n").collect();

    let path = str_vec.first().unwrap().split_whitespace().nth(1).unwrap();

    path.to_string()
}
