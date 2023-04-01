mod thread_pool;

use std::env;
use tokio::io;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::queue;
use crate::jobs::{Jobs, print_message, JobData, print_sum};
use crate::queue::queue::Queue;
use crate::worker::thread_pool::ThreadPool;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

enum Message {
    NewJob(Job),
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock()
                .expect("Can't block the current thread")
                .recv()
                .expect("Channel has hang up");

            match message {
                Message::NewJob(job) => {
                    job();
                }
            }
        });

        Worker { _id: id, _thread: thread }
    }
}

pub(crate) async fn execute(queue: Arc<queue::FifoQueue>) -> io::Result<()> {
    let pool = ThreadPool::new(
        env::var("WORKERS")
            .expect("Number of workers not set")
            .parse()
            .expect("Can't parse the number of workers")
    );
    loop {
        let job = queue.pop();
        match job {
            Some(job) => {
                pool.execute(move || {
                    let jobs: Jobs = serde_json::from_str(job.job.as_str())
                        .expect("Can't deserialize");
                    
                    match jobs {
                        Jobs::PrintMessageJob { data } =>
                            match print_message::handle(JobData { _id: job.id, _status: job.status, data }) {
                                Ok(_) => (),
                                Err(_) => (),
                            },
                        Jobs::PrintSumJob { data } =>
                            match print_sum::handle(JobData { _id: job.id, _status: job.status, data }) {
                                Ok(_) => (),
                                Err(_) => (),
                            },
                    };
                });
            }
            _ => {}
        };
    }
}
