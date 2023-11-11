// Uncomment this block to pass the first stage
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
enum Route {
    BASE,
    ECHO,
    USERAGENT,
    NOTFOUND,
}

struct Headers {
    method: String,
    path: String,
    user_agent: String,
}

impl Headers {
    fn new() -> Self {
        Self {
            method: String::new(),
            path: String::new(),
            user_agent: String::new(),
        }
    }

    fn set_method(&mut self, method: String) {
        self.method = method;
    }
    fn set_path(&mut self, path: String) {
        self.path = path;
    }
    fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let headers = parse_headers(buffer);
    route_handler(headers, stream);
}

fn route_handler(headers: Headers, mut stream: TcpStream) {
    let path = &headers.path;
    match path.as_str() {
        "/" => {
            stream
                .write(get_content(Route::BASE, &headers).as_bytes())
                .unwrap();
        }
        route => {
            if route.starts_with("/echo") {
                stream
                    .write(get_content(Route::ECHO, &headers).as_bytes())
                    .unwrap();
            } else if route.starts_with("/user-agent") {
                stream
                    .write(get_content(Route::USERAGENT, &headers).as_bytes())
                    .unwrap();
            } else {
                stream
                    .write(get_content(Route::NOTFOUND, &headers).as_bytes())
                    .unwrap();
            }
        }
    }
    stream.flush().unwrap();
}

fn get_content(route: Route, headers: &Headers) -> String {
    match route {
        Route::BASE => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        Route::ECHO => {
            let body_res = &headers.path.to_string().replace("/echo/", "");
            let content_len = body_res.len();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                content_len, body_res
            )
        }
        Route::USERAGENT => {
            let body_res = &headers.user_agent;
            let content_len = body_res.len();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                content_len, body_res
            )
        }
        Route::NOTFOUND => "HTTP/1.1 404 NOT FOUND\r\n\r\nNOT FOUND".to_string(),
    }
}

fn parse_headers(buffer: [u8; 1024]) -> Headers {
    let buf_to_string = String::from_utf8_lossy(&buffer);
    let str_vec: Vec<&str> = buf_to_string.split("\r\n").collect();
    let mut start_line = str_vec.first().unwrap().split_whitespace();
    let path = start_line.nth(1).unwrap().to_string();
    let method = start_line.nth(0).unwrap().to_string();
    let mut headers = Headers::new();
    headers.set_path(path);
    headers.set_method(method);

    let user_agent = str_vec.iter().find(|e| e.starts_with("User-Agent:"));
    if let Some(agent) = user_agent {
        headers.set_user_agent(agent.replace("User-Agent: ", ""));
    }
    headers
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let port = "4221";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    println!("accepted new connection");
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
