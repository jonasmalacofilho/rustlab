use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let workers = (0..size).map(|id| Worker::new(id)).collect::<Vec<_>>();

        ThreadPool { workers }
    }

    pub fn execute(&self, task: impl FnOnce() -> () + Send + 'static) {
        thread::spawn(task);
    }
}

struct Worker {
    id: u32,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
