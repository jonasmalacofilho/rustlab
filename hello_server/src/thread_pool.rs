// things to consider:
// - must monitor pool capacity/queue size
// - worker threads may panic and may need to be restarted
//
// next idea to try:
// * wu count:
//   - pool increments wu count
//   - worker decrements wu count
//   - pool checks wu count when accepting a new job
//   - panicking worker decrements wu count
// * worker count:
//   - pool starts with initial workers and worker count
//   - panicking worker decrements worker count
//   - pool checks worker count when accepting a new job;
//     if necessary, starts a new worker

use std::collections::VecDeque;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

type WorkUnit = Box<dyn FnOnce() -> () + Send + 'static>;

struct Worker {
    tx: Sender<WorkUnit>,
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(feedback: Sender<()>) -> Worker {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let wu: WorkUnit = rx.recv().unwrap();
            wu();
            feedback.send(()).unwrap();
        });

        Worker { tx, handle }
    }
}

pub struct ThreadPool {
    free: VecDeque<Worker>,
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        let (tx, rx) = mpsc::channel();
        let mut free = VecDeque::new();
        for i in 0..size {
            free.push_back(Worker::new(tx.clone()));
        }
        ThreadPool { free }
    }

    /// Submit a `f` work unit to the pool.
    ///
    /// If the pool capacity has not been exhausted, the job is accepted and this method returns
    /// `Ok(())`.  Otherwise, the job is refused and this method returns `Err(())`.
    pub fn submit<F>(&self, f: F) -> Result<(), ()>
    where
        F: FnOnce() -> (),
        F: Send + 'static,
    {
        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        use std::sync::{Arc, Condvar, Mutex};

        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        let pool = ThreadPool::new(2);
        pool.submit(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        })
        .unwrap();

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}
