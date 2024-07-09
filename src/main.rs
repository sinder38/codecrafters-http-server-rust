mod parser;
use crate::HttpConnectionError::NotFound;
use parser::HttpRequest;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs:");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
#[derive(PartialEq)]
enum HttpConnectionError {
    NotFound,
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let http_request = HttpRequest::parse(request);
    match http_request {
        Ok(request) => {
            println!("{:#?}", request);
            let mut split_req_target = request.target.split("/");
            if split_req_target.next().ok_or(NotFound) == Ok("") {
                match split_req_target.next() {
                    None => {
                        stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                    } //TODO should be unreachable
                    Some("") => {
                        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    }
                    Some("echo") => {
                        echo(stream, split_req_target.next());
                    }
                    Some(_) => {
                        stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                    }
                }
            }
        }
        Err(_) => {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
        }
    }
}

fn echo(mut stream: TcpStream, echo_message: Option<&str>) {
    match echo_message {
        None => {
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
        }
        Some(text) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                text.len(),
                text
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

