use std::io::prelude::*;
use std::io::BufReader;

use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let pretty_peer = match stream.peer_addr() {
        Ok(a) => a.to_string(),
        Err(_) => String::from("unknown peer"),
    };
    eprintln!("{} from {}", request_line.trim_end(), pretty_peer);

    let hello = "GET / ";
    let sleep = "GET /sleep ";

    let (status_line, contents) = if request_line.starts_with(hello) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else if request_line.starts_with(sleep) {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "404.html")
    };

    let contents = fs::read(contents)?;

    stream.write(status_line.as_bytes())?;

    stream.write(format!("Content-Length: {}\r\n\r\n", contents.len()).as_bytes())?;
    stream.write(&contents)?;

    stream.flush()?;

    eprintln!("{}", status_line);

    Ok(())
}
