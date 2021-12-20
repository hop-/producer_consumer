use std::sync::{Arc, Mutex, Condvar};

pub struct Writer {
    queue: Arc<Mutex<Vec<u32>>>,
    push_cv: Arc<Condvar>,
    pop_cv: Arc<Condvar>,
    threshold: usize,
}

impl Writer {
    pub(super) fn new(
        queue: Arc<Mutex<Vec<u32>>>,
        push_cv: Arc<Condvar>,
        pop_cv: Arc<Condvar>,
        threshold: usize,
    ) -> Writer {
        Writer {
            queue: queue,
            push_cv: push_cv,
            pop_cv: pop_cv,
            threshold: threshold,
        }
    }

    pub fn push(&self, value: u32) {
        let _ = self.push_cv.wait_while(self.queue.lock().unwrap(), |q| { q.len() >= self.threshold }).unwrap();

        self.queue.lock().unwrap().push(value);

        self.pop_cv.notify_all();
    }
}