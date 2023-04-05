use std::{net::TcpListener, io::{Read, Write}};
fn main() {

    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    println!("start on 9000 ....");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        stream.write(&mut buf).unwrap();
    }
    println!("Hello, world!");
}
