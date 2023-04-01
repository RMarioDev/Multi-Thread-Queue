use dotenv::dotenv;
use std::sync::Arc;
use crate::queue::queue::Queue;

mod queue;
mod worker;
mod tcp_stream;
mod jobs;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let queue = Arc::new(queue::FifoQueue::new());
    let q1 = queue.clone();
    let q2 = queue.clone();
    
    tokio::spawn(async move {
        worker::execute(q1).await
    });
    
    tcp_stream::listen(q2);
}
