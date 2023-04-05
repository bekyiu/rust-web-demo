use std::{net::TcpStream, io::{Write, Read}};

fn main() {
    let mut stream = TcpStream::connect("localhost:9000").unwrap();
    stream.write("你好".as_bytes()).unwrap();
    let mut buf = [0; 10];
    stream.read(&mut buf).unwrap();
    println!("收到了服务器的响应: {:?}", std::str::from_utf8(&buf).unwrap());
}
