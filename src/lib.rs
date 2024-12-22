use core::panic;
use std::thread;

pub const BIND_ADDRESS: &str = "127.0.0.1:7878";

pub struct ThreadPool {
    workers: Vec<Worker>,
}

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

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        println!("Creating worker {id}");
        Worker { id, thread }
    }
}
