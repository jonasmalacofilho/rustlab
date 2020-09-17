use std::io::prelude::*;
use std::io::BufReader;

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
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let pretty_peer = match stream.peer_addr() {
        Ok(a) => a.to_string(),
        Err(_) => String::from("unknown peer"),
    };
    eprintln!("{} from {}", request_line.trim_end(), pretty_peer);

    let (status_line, contents) = if request_line.starts_with("GET / HTTP") {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "404.html")
    };

    let contents = fs::read(contents).unwrap();

    stream.write(status_line.as_bytes()).unwrap();

    stream.write(format!("Content-Length: {}\r\n\r\n", contents.len()).as_bytes()).unwrap();
    stream.write(&contents).unwrap();

    stream.flush().unwrap();

    eprintln!("{}", status_line);
}
