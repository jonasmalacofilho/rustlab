use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
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
    fn read(stream: TcpStream) -> io::Result<Request> {
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
    InternalServerError,
    TemporaryRedirect(String),
}

const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";

impl Response {
    fn to_string(&self) -> io::Result<String> {
        match self {
            Response::Ok(content) => Ok(format!("HTTP/1.1 200 OK\r\n\r\n{}", content)),
            Response::NotFound => {
                let content = fs::read_to_string("404.html")?;
                Ok(format!("HTTP/1.1 400 OK\r\n\r\n{}", content))
            }
            Response::InternalServerError => Ok(String::from(INTERNAL_SERVER_ERROR)),
            Response::TemporaryRedirect(uri) => Ok(format!(
                "HTTP/1.1 307 Temporary Redirect\r\nLocation: {}\r\n\r\n",
                uri
            )),
        }
    }
}

fn handle_stream(stream: TcpStream) -> io::Result<Response> {
    let req = Request::read(stream)?;
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
    Ok(response)
}

fn handle_stream_in_worker(stream: io::Result<TcpStream>) -> () {
    let mut stream = stream.unwrap();
    let response =
        handle_stream(stream.try_clone().unwrap()).unwrap_or(Response::InternalServerError);
    // In a real program we would handle `stream.write` and `stream.flush` errors more gracefully,
    // by retrying and/or logging the issue before returning
    stream
        .write(
            &dbg!(response)
                .to_string()
                .unwrap_or(String::from(INTERNAL_SERVER_ERROR))
                .as_bytes(),
        )
        .unwrap();
    stream.flush().unwrap();
}

mod thread_pool {
    use std::thread::{self, JoinHandle};

    struct Worker {
        free: bool,
        handle: JoinHandle<()>,
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
    }

    impl ThreadPool {
        pub fn new(size: u32) -> ThreadPool {
            // a manager thread monitors received 
            ThreadPool { workers: vec![] }
        }

        /// Submit a `f` job to the pool.
        ///
        /// If there is at least one free worker, this function will return `Ok(())` immediately
        /// and one of the free workers will take and run the job.
        ///
        /// If there are no free workers, the job is refused and this function returns an
        /// `Err(())`.
        pub fn submit<F>(&self, f: F) -> Result<(), ()>
        where
            F: FnOnce() -> (),
            F: Send + 'static,
        {
            for w in &self.workers {
                if w.free {
                    // TODO send job
                    w.handle.thread().unpark();
                    return Ok({});
                }
            }
            Err(())
        }
    }
}

fn main() {
    let pool = thread_pool::ThreadPool::new(100);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        pool.submit(|| handle_stream_in_worker(stream))
            .unwrap_or_else(|_| eprintln!("Cannot handle request"));
    }
}
