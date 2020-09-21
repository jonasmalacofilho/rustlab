mod thread_pool;

use std::io::prelude::*;
use std::io::BufReader;

use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::fs;
use std::thread;
use std::time::Duration;

use uuid::Uuid;

use signal_hook::iterator::Signals;

use thread_pool::ThreadPool;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut exit = 0;

    {
        let bind_addr = "0.0.0.0:7878";

        let alive = Arc::new(AtomicBool::new(true));

        let listener = {
            let alive = Arc::clone(&alive);

            thread::spawn(move || {
                if std::panic::catch_unwind(move || listen(bind_addr, alive).unwrap()).is_err() {
                    std::process::abort();
                }
            })
        };

        let term_signals = [
            signal_hook::SIGTERM,
            signal_hook::SIGINT,
            signal_hook::SIGQUIT,
        ];

        if let Some(signal) = Signals::new(&term_signals)?.forever().next() {
            eprintln!("received signal to terminate: {}", signal);
            exit = 128 + signal;
        }

        // ensure a second signal results in default/immediate termination
        for signal in &term_signals {
            signal_hook::cleanup::cleanup_signal(*signal)?;
        }

        // stop processing new requests
        alive.store(false, Ordering::SeqCst);

        // send dummy request to unblock (if necessary) the listener thread
        let _ = TcpStream::connect(bind_addr);

        eprintln!("will try to wait for any in-progress requests to terminate");
        listener.join().expect("listener had already panicked");
    }

    eprintln!("goodbye and thanks for all the fish");
    std::process::exit(exit);
}

fn listen<A: ToSocketAddrs>(bind_addr: A, alive: Arc<AtomicBool>) -> Result<()> {
    let listener = TcpListener::bind(bind_addr)?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        if !alive.load(Ordering::SeqCst) {
            break;
        }

        let stream = stream?; // FIXME should not terminate

        pool.execute(|| {
            handle_connection(stream).unwrap();
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
    let panic = "GET /panic ";

    let (status_line, contents) = if request_line.starts_with(hello) {
        ("HTTP/1.1 200 OK\r\n", Some("hello.html"))
    } else if request_line.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", None)
    } else if request_line.starts_with(panic) {
        panic!("Oh no!!!");
    } else {
        ("HTTP/1.1 404 Not Found\r\n", Some("404.html"))
    };

    stream.write_all(status_line.as_bytes())?;

    if let Some(contents) = contents {
        let contents = fs::read(contents)?;
        stream.write_all(format!("Content-Length: {}\r\n\r\n", contents.len()).as_bytes())?;
        stream.write_all(&contents)?;
    } else {
        stream.write_all(b"\r\n\r\n")?;
    }

    stream.flush()?;

    eprintln!("[{}] > {}", request_id, status_line.trim_end());

    Ok(())
}
