// src/storage/api_server/server.rs

use crate::storage::api_server::router::Router;
use crate::storage::api_server::handler::Handler;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct APIServer {
    listener: TcpListener,
    router: Router,
}

impl APIServer {
    pub fn new(addr: &str, router: Router) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener, router })
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let router = self.router.clone();
            thread::spawn(move || {
                handle_connection(stream, router);
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Router) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    if let Some(handler) = router.route(&request) {
        let response = handler.handle(&request);
        stream.write(response.as_bytes()).unwrap();
    } else {
        let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(not_found_response.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}
