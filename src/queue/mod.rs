use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::queue::Queue;

pub mod queue;

#[derive(Debug)]
pub struct FifoQueue {
    data: Mutex<VecDeque<QueueJob>>,
    cv: Condvar,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum JobStatus {
    Queued,
    Running,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueueJob {
    pub(crate) id: Uuid,
    pub(crate) job: String,
    pub(crate) created_at: chrono::DateTime<Utc>,
    pub(crate) updated_at: chrono::DateTime<Utc>,
    pub(crate) status: JobStatus,
}

impl Queue for FifoQueue {
    fn new() -> Self {
        Self {
            data: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        }
    }

    fn push(&self, job: QueueJob) {
        let mut data = self.data.lock().unwrap();
        data.push_back(job);
        self.cv.notify_one();
    }

    fn pop(&self) -> Option<QueueJob> {
        let mut data = self.data.lock().unwrap();

        while data.is_empty() {
            data = self.cv.wait(data).unwrap();
        }

        data.pop_front()
    }


    fn len(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }

    fn is_empty(&self) -> bool {
        let data = self.data.lock().unwrap();
        data.is_empty()
    }
}
