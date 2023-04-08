use std::{net::TcpListener, io::Read};

use http::http_request::HttpRequest;
use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self { socket_addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("start on: {}", self.socket_addr);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("connection established");

            // 图省事 写大点
            let mut buf = [0; 4096];
            stream.read(&mut buf).unwrap();
            let req: HttpRequest = String::from_utf8(buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
