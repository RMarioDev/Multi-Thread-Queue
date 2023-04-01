use std::sync::{Arc, mpsc, Mutex};
use crate::worker::{Message, Worker};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(max_workers: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(max_workers);

        for id in 0..max_workers {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { _workers: workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(Message::NewJob(job)).expect("Can't send a value to the channel");
    }
}
