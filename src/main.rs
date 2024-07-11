mod http_response;
mod parser;

use std::ops::Deref;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::http_response::{
    HttpResponse, HttpResponseStatus, PLAIN_TEXT_CONTENT_TYPE, USER_AGENT_KEY,
};
use crate::parser::HttpRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Logs:");

    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    loop {
        // Accept a new socket
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap(); // TODO remove this unwrap

    let binding = String::from_utf8_lossy(&buffer[..]);
    let request = binding.deref(); // Not sure if this is the best way to do this. maybe should just clone
    let http_request = HttpRequest::from(request);
    match http_request {
        Ok(r) => {
            println!("{:#?}", r);
            match r.method {
                "GET" => {
                    stream.write_all(&process_get(r)).await.unwrap();
                }
                _ => {
                    panic!("Unexpected method")
                }
            }
        }
        Err(_) => {
            stream
                .write_all(
                    &HttpResponse::new(HttpResponseStatus::NotFound, None, None, None).as_bytes(),
                )
                .await
                .unwrap();
        }
    }
}
fn process_get(http_request: HttpRequest) -> Vec<u8> {
    let mut depth: usize = 0;
    // Match target
    match http_request.uri.get(depth) {
        Some(&"/") => {
            depth += 1;
            match http_request.uri.get(depth) {
                None => {
                    println!("responding with pong");
                    HttpResponse::new(HttpResponseStatus::Ok, None, None, None).as_bytes()
                }
                Some(&"echo/") => respond_echo(http_request, depth).as_bytes(),
                Some(&"user-agent") => respond_user_agent(http_request, depth).as_bytes(),
                Some(_) => {
                    // b"HTTP/1.1 404 Not Found\r\n\r\n"
                    HttpResponse::new(HttpResponseStatus::NotFound, None, None, None).as_bytes()
                }
            }
        }
        None | Some(_) => {
            //TODO should be unreachable
            HttpResponse::new(HttpResponseStatus::NotFound, None, None, None).as_bytes()
        }
    }
}
fn respond_echo(http_request: HttpRequest, depth: usize) -> HttpResponse {
    println!("responding with echo");
    match http_request.uri.get(depth + 1) {
        None => {
            // b"HTTP/1.1 200 OK\r\n\r\n"
            HttpResponse::new(HttpResponseStatus::Ok, None, None, None)
        }
        Some(echo_message) => HttpResponse::new(
            HttpResponseStatus::Ok,
            None,
            Some(echo_message.to_string()),
            Some(PLAIN_TEXT_CONTENT_TYPE.to_string()),
        ),
    }
}

fn respond_user_agent(http_request: HttpRequest, depth: usize) -> HttpResponse {
    match http_request.uri.get(depth + 1) {
        None => {
            println!("responding with user agent");
            let user_agent = http_request.headers.get(USER_AGENT_KEY).unwrap_or(&"");
            HttpResponse::new(
                HttpResponseStatus::Ok,
                None,
                Some(user_agent.to_string()),
                Some(PLAIN_TEXT_CONTENT_TYPE.to_string()),
            )
        }
        Some(_) => {
            // /user-agent is supposed to be the endpoint
            HttpResponse::new(HttpResponseStatus::NotFound, None, None, None)
        }
    }
}
