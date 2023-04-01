use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::queue::JobStatus;

pub(crate) mod print_message;
pub(crate) mod print_sum;

#[derive(Debug, Serialize, Deserialize)]
pub enum Jobs {
    PrintMessageJob {
        data: HashMap<String, String>,
    },
    PrintSumJob {
        data: HashMap<String, String>,
    }
}

pub struct JobData {
    pub(crate) _id: Uuid,
    pub(crate) _status: JobStatus,
    pub(crate) data: HashMap<String, String>,
}
