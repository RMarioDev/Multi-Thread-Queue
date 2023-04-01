use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener};
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use crate::queue::{FifoQueue, JobStatus, QueueJob};
use crate::queue::queue::Queue;

pub(crate) fn listen(queue: Arc<FifoQueue>) {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .expect("TCP: listener not working");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Stream not working");
        let mut buffer_size = [0; 1024];
        let usize = stream.read(&mut buffer_size).expect("TCP: Can't read a message");

        if usize > 0 {
            let job = QueueJob {
                id: Uuid::new_v4(),
                job: String::from_utf8_lossy(&buffer_size[0..usize]).to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                status: JobStatus::Queued,
            };

            queue.push(job);
            stream.write(&buffer_size).expect("TCP: Can't write a message");
        }
    }
}
