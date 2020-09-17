use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    eprintln!(
        "Connection established with {}",
        stream.peer_addr().unwrap()
    );

    let mut buf = [0; 4096];
    stream.read(&mut buf).unwrap();

    eprintln!("{}", String::from_utf8_lossy(&buf[..]));

    let contents = fs::read("hello.html").unwrap();

    let res = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", contents.len());
    stream.write(res.as_bytes()).unwrap();

    stream.write(&contents).unwrap();

    stream.flush().unwrap();
}
