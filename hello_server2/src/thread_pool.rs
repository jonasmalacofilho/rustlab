use std::panic::{self, UnwindSafe};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Task = Box<dyn FnOnce() + Send + UnwindSafe + 'static>;

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

    pub fn execute(&self, task: impl FnOnce() + Send + UnwindSafe + 'static) {
        let task = Box::new(task);
        let message = Message::Execute(task);
        self.sender.send(message).expect("broken channel");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("broken channel");
        }

        for Worker { id, thread } in self.workers.drain(..) {
            let panicked = thread.join().is_err();

            if panicked {
                eprintln!("worker #{} had failed prematurely", id);
            }
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: u32,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::Builder::new()
            .name(format!("worker #{}", id))
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv().expect("broken channel");

                match message {
                    Message::Execute(task) => {
                        let outcome = panic::catch_unwind(task);

                        if outcome.is_err() {
                            eprintln!(
                                "panic caught, {} still alive",
                                thread::current().name().expect("unnamed worker thread")
                            );
                        }
                    }
                    Message::Terminate => break,
                }
            })
            .expect("could not spawn worker thread");

        Worker { id, thread }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn drop_waits_for_pending_tasks() {
        use std::time::Duration;

        // create a mutually exclusive flag and immediately take the lock
        let flag = Arc::new(Mutex::new(false));
        let guard = flag.lock().unwrap();

        // submit a task that flips that flag to a pool; as the lock is taken, this will not run
        // for now
        let pool = ThreadPool::new(4);
        let flag2 = Arc::clone(&flag);
        pool.execute(move || {
            let mut flag = flag2.lock().unwrap();
            *flag = true;
        });

        // spawn an accessory thread to drop the pool, because we can't block ourselves
        let helper = thread::spawn(move || drop(pool));

        // sensibleness check that the flag is false
        assert!(!*guard);

        // this is not necessary; but, just to be extra safe, wait for a bit: if there was a race,
        // this would make the wrong behavior more likely (and, thus, make it easier to spot)
        thread::sleep(Duration::from_millis(100));

        // allow the previous submitted task to take the lock and flip the flag
        drop(guard);

        // wait for the pool to be dropped
        helper.join().expect("drop has panicked");

        // check that the task was able to flip the flag, even though the pool was being drooped
        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn survives_panics() {
        let flag = Arc::new(Mutex::new(false));

        let pool = ThreadPool::new(1);
        pool.execute(|| panic!("simulated panic"));

        let flag2 = Arc::clone(&flag);
        pool.execute(move || {
            let mut flag = flag2.lock().unwrap();
            *flag = true;
        });

        // wait for the pending tasks by dropping the pool
        drop(pool);

        assert!(*flag.lock().unwrap());
    }
}
