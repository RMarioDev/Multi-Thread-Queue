use crate::queue::QueueJob;

pub trait Queue {
    fn new() -> Self;
    
    fn push(&self, value: QueueJob);

    fn pop(&self) -> Option<QueueJob>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}
