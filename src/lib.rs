use core::panic;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub const BIND_ADDRESS: &str = "127.0.0.1:7878";

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            } else {
                println!(
                    "Tried shutting down worker {}, but it was already shutdown.",
                    worker.id
                );
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Error acquiring MutexGuard, maybe it is in a poisoned state?")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing...");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} got disconnected; shutting down...");
                    break;
                }
            }
        });

        println!("Creating worker {id}");
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
