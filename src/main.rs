use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn fallback_error(err: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

#[derive(Debug)]
struct Request {
    method: Method,
    uri: String,
    version: Version,
}

#[derive(Debug, PartialEq)]
enum Method {
    Get,
    Post,
}

#[derive(Debug)]
enum Version {
    Http1_1,
    Http2_0,
}

impl Request {
    fn read(stream: TcpStream) -> Result<Request> {
        // Properly parsing HTTP is outside of the scope of this code
        let reader = BufReader::new(stream);
        let request_line = reader
            .lines()
            .next()
            .ok_or(fallback_error("Empty request"))??;
        let mut parts = dbg!(&request_line).split(' ');
        let method = match parts.next() {
            Some("GET") => Method::Get,
            Some("POST") => Method::Post,
            _ => return Err(fallback_error("Invalid or missing request method")),
        };
        let uri = parts
            .next()
            .ok_or(fallback_error("Missing request URI"))?
            .to_string();
        let version = match parts.next() {
            Some("HTTP/1.1") => Version::Http1_1,
            Some("HTTP/2") => Version::Http2_0,
            _ => return Err(fallback_error("Invalid or missing HTTP version")),
        };
        Ok(Request {
            method,
            uri,
            version,
        })
    }
}

#[derive(Debug)]
enum Response {
    Ok(String),
    NotFound,
    TemporaryRedirect(String),
}

impl Response {
    fn to_string(&self) -> Result<String> {
        match self {
            Response::Ok(content) => Ok(format!("HTTP/1.1 200 OK\r\n\r\n{}", content)),
            Response::NotFound => {
                let content = fs::read_to_string("404.html")?;
                Ok(format!("HTTP/1.1 400 OK\r\n\r\n{}", content))
            }
            Response::TemporaryRedirect(uri) => Ok(format!(
                "HTTP/1.1 307 Temporary Redirect\r\nLocation: {}\r\n\r\n",
                uri
            )),
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let req = Request::read(stream.try_clone()?)?;
    let response = match req {
        Request {
            method: Method::Get,
            uri,
            ..
        } if uri == "/" => {
            let contents = fs::read_to_string("hello.html")?;
            Response::Ok(contents)
        }
        Request {
            method: Method::Get,
            uri,
            ..
        } if uri == "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            Response::TemporaryRedirect(String::from("/"))
        }
        _ => Response::NotFound,
    };
    stream.write(&dbg!(response).to_string()?.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        handle_stream(stream?)?;
    }
    Ok(())
}
