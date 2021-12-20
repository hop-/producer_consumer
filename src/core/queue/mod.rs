use crate::core::queue::queue::Queue;
use crate::core::queue::reader::Reader;
use crate::core::queue::writer::Writer;

use std::sync::{Arc, Mutex, Condvar};

pub mod queue;
pub mod reader;
pub mod writer;

pub fn create(size: usize) -> (Writer, Reader, Queue) {
    let queue = Arc::new(Mutex::new(Vec::new()));
    
    let push_cv = Arc::new(Condvar::new());
    let pop_cv = Arc::new(Condvar::new());
    
    let push_start_threshold = size * 80 / 100;
    
    (
        Writer::new(queue.clone(), push_cv.clone(), pop_cv.clone(), size),
        Reader::new(queue.clone(), push_cv.clone(), pop_cv, push_start_threshold),
        Queue::new(queue, push_cv),
    )
}