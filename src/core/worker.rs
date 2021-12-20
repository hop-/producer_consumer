use std::sync::mpsc;
use std::thread;

pub struct Worker {
    sender: Option<mpsc::Sender<()>>,
    handler: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            sender: None,
            handler: None,
        }
    }
}

impl Worker {
    pub fn start<F>(&mut self, job: F)
    where
        F: 'static + Send + Fn() -> ()
    {
        //TODO: check if already started
        let (s, r) = mpsc::channel();
        self.sender = Some(s);

        self.handler = Some(thread::spawn(move || {
            loop {
                job();
                match r.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    _ => {}
                }
            }
        }));
    }

    pub fn stop(&self) {
        self.sender.as_ref().unwrap().send(()).unwrap();
    }

    pub fn wait(&mut self) {
        self.handler.take().unwrap().join().unwrap();
    }
}