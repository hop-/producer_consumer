use std::sync::{Arc, Mutex, Condvar};

pub struct Reader {
    queue: Arc<Mutex<Vec<u32>>>,
    push_cv: Arc<Condvar>,
    pop_cv: Arc<Condvar>,
    threshold: usize,
}

impl Reader {
    pub(super) fn new(
        queue: Arc<Mutex<Vec<u32>>>,
        push_cv: Arc<Condvar>,
        pop_cv: Arc<Condvar>,
        threshold: usize,
    ) -> Reader {
        Reader {
            queue: queue,
            push_cv: push_cv,
            pop_cv: pop_cv,
            threshold: threshold,
        }
    }

    pub fn pop(&self) -> u32 {
        let _ = self.pop_cv.wait_while(self.queue.lock().unwrap(), |q| { q.len() == 0 }).unwrap();

        let mut queue = self.queue.lock().unwrap();
        let value = queue.pop().unwrap();

        if queue.len() < self.threshold {
            self.push_cv.notify_all();
        }

        value
    }
}