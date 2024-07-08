mod parser;
use parser::HttpRequest;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                handle_connection(stream);

            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let http_request = HttpRequest::parse(request);
    match http_request {
        Ok(request)=>{
            println!("{:#?}",request);
            if request.target == "/index.html" || request.target == "/" {
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            } else {
                stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
            }
        },
        Err(_) => {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
        }
    }
}
