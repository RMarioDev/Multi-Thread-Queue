# Job Queue System and Workers

### An application in rust accepts jobs via the TCP protocol and processes them in a queue. With the help of a worker, you can spawn n threads (where n you specify in the env file).

#### There is also an additional app (container sender) in rust where you can test sending jobs to the queue and processing jobs.

VecDeque is currently used for the queue and therefore the queue is intended exclusively for VecDeque use. This does not support the system updating job status in the queue and retrying on n failed executes. After consideration and research, I think we need some correct data persistent storage for anything more to work with the queue. database. It is desirable that the queue can be managed with queries (SQL) and stored somewhere suitable.

## Docker

#### If you want, you can run the application in docker using the docker compose file. All you need locally is docker (https://www.docker.com/) and rustup (https://rustup.rs/).

Docker has two containers that are created on the same rust image. The queue container is on port 3333 and the sender container is on port 80. In both containers, we run cargo watch (Cargo Watch watches over your project's source for changes, and runs Cargo commands when they occur). The Docker file is located at ```root/config/rust.dockerfile```

If you don't use docker, you can also run the app locally, which should be behind port 3333, and the sender app which can be on any port and only needs access to port 3333.

## TCP socket server

#### With the help of tokio (https://tokio.rs/), there is an asynchronous app that continuously listens for the incoming stream.

```rust
let listener = TcpListener::bind(format!("{}:{}", host, port))
.expect("TCP: listener not working");

for stream in listener.incoming()
```

## Queue

We implement fifo VecDeque which is Mutex (protecting shared data) protected and contains multiple ```QueueJobs```The queue is multi thread and also protected with Condvar (block a thread).

```rust
pub struct FifoQueue {
    data: Mutex<VecDeque<QueueJob>>,
    cv: Condvar,
}
```

The interface for the queue (```impl Queue for FifoQueue```)

```rust
fn new() -> Self;
fn push(&self, value: QueueJob);
fn pop(&self) -> Option<QueueJob>;
fn len(&self) -> usize;
fn is_empty(&self) -> bool;
```

For transfer a messages, we use a buffer ```[0; 1024]```

With the TCP protocol, we receive incoming messages and send them to a queue.

The queue is populated with the structs ```QueueJob ```which is serialized, and it is necessary to ensure that the sender always sends the correct/same format of the serialized struct.

```rust
pub enum Jobs {
    PrintMessageJob {
        data: HashMap<String, String>,
    },
    PrintSumJob {
        data: HashMap<String, String>,
    }
}
```

## Jobs

#### We have two test jobs that are registered as modules.

```rust
pub(crate) mod print_message;
pub(crate) mod print_sum;
```

Data, id and job status are available on the job. The first job sleeps for 2 seconds and then outputs a message, and the second job also sleeps for 2 seconds and then outputs the sum of two numbers.

```rust
queue   | Result: My 0 job
queue   | Result: Sum of the number 0 and 0 is 0
queue   | Result: My 1 job
queue   | Result: Sum of the number 1 and 1 is 2
```

## Sender

#### A loop with which we send 8 jobs via the TCP protocol. We send two different test jobs and receive a return message.

```rust
for i in 0..4 {
    add_to_print_message_job(i);
    add_to_print_sum_job(i);
}
```

## Worker

We spawn a new asynchronous task and send them a queue. Task only ensures that the task runs in a loop and checks for active unprocessed jobs.

```rust
tokio::spawn(async move {
    worker::execute(q1)
});
```

For each necessary job, n configured workers are available, which use the ```channel``` to listen for necessary tasks when they are available. Each worker listens and waits in his thread.

```rust
for id in 0..max_workers {
workers.push(Worker::new(id, Arc::clone(&receiver)));
}
```

When a job is sent to a thread, a free worker takes it over and processes it in such a way as to execute its callback, which is the content of the individual job.
