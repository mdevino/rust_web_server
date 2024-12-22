use core::panic;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub const BIND_ADDRESS: &str = "127.0.0.1:7878";

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPoll.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("ThreadPoll size cannot be 0.");
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Error acquiring MutexGuard, maybe it is in a poisoned state?")
                .recv()
                .unwrap();
            println!("Worker {id} got a job; executing...");
            job();
        });

        println!("Creating worker {id}");
        Worker { id, thread }
    }
}
