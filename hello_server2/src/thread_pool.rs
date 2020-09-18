use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Task = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Task>,
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, receiver.clone()))
            .collect::<Vec<_>>();

        ThreadPool { workers, sender }
    }

    pub fn execute(&self, task: impl FnOnce() + Send + 'static) {
        self.sender.send(Box::new(task)).unwrap();
    }
}

#[allow(dead_code)]
struct Worker {
    id: u32,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();

            task();
        });

        Worker { id, thread }
    }
}
