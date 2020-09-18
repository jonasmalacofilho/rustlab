use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Task = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Execute(Task),
    Terminate,
}

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
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
        let task = Box::new(task);
        let message = Message::Execute(task);
        self.sender.send(message).unwrap();
    }
}

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         for worker in &self.workers {
//         }
//     }
// }

#[allow(dead_code)]
struct Worker {
    id: u32,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Execute(task) => task(),
                Message::Terminate => break,
            }
        });

        Worker { id, thread }
    }
}
