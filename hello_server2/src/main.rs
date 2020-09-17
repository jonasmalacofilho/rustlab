mod thread_pool;

use std::io::prelude::*;
use std::io::BufReader;

use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::thread;
use std::time::Duration;

use uuid::Uuid;

use thread_pool::ThreadPool;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|| {
            let _ = handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request_id = Uuid::new_v4();

    let mut reader = BufReader::new(stream.try_clone()?);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let pretty_peer = match stream.peer_addr() {
        Ok(a) => a.to_string(),
        Err(_) => String::from("unknown peer"),
    };
    eprintln!(
        "[{}] < {} from {}",
        request_id,
        request_line.trim_end(),
        pretty_peer
    );

    let hello = "GET / ";
    let sleep = "GET /sleep ";

    let (status_line, contents) = if request_line.starts_with(hello) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else if request_line.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n", "404.html")
    };

    let contents = fs::read(contents)?;

    stream.write(status_line.as_bytes())?;

    stream.write(format!("Content-Length: {}\r\n\r\n", contents.len()).as_bytes())?;
    stream.write(&contents)?;

    stream.flush()?;

    eprintln!("[{}] > {}", request_id, status_line.trim_end());

    Ok(())
}
