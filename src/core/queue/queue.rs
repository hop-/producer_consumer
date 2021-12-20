use std::sync::{Arc, Mutex, Condvar};

pub struct Queue{
    queue: Arc<Mutex<Vec<u32>>>,
    push_cv: Arc<Condvar>,
}

impl Queue{
    pub(super) fn new(
        queue: Arc<Mutex<Vec<u32>>>,
        push_cv: Arc<Condvar>
    ) -> Queue{
        Queue{
            queue: queue,
            push_cv: push_cv,
        }
    }

    pub fn wait_until_empty(&self) {
        let _ = self.push_cv.wait_while(self.queue.lock().unwrap(), |q| { q.len() > 0 }).unwrap();
    }

    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}