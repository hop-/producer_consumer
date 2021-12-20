use crate::core::data_file::DataFile;
use crate::core::worker::Worker;
use crate::core::queue;

use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use ctrlc;
use rand::Rng;

const MIN_PRODUCERS: usize = 1;
const MAX_PRODUCERS: usize = 10;
const MIN_CONSUMERS: usize = 1;
const MAX_CONSUMERS: usize = 10;

const DATA_FILE_NAME: &str = "data.txt";
const QUEUE_SIZE: usize = 100;

const WAIT_INTERVAL_MS: u64 = 1000;

fn input_number(name: &str, min: usize, max: usize) -> usize {
    let mut input_text = String::new();
    loop {
        input_text.clear();
        println!("Please input the count of {} ({}-{})", name, min, max);
        std::io::stdin().read_line(&mut input_text).unwrap();
        let number = input_text.trim().parse().unwrap();
        if number >= min && number <= max {
            return number;
        }
    }
}

fn sleep(duration: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration))
}

// Application

pub struct Application {
    int_sender: Arc<Mutex<mpsc::Sender<()>>>,
    int_recv: mpsc::Receiver<()>,
    out_file: Arc<Mutex<DataFile>>,
    data_queue: queue::queue::Queue,
    q_sender: Arc<Mutex<queue::writer::Writer>>,
    q_receiver: Arc<Mutex<queue::reader::Reader>>,
    consumers: Vec<Worker>,
    producers: Vec<Worker>,
}

impl Application {
    pub fn new() -> Application {
        let (s, r, q) = queue::create(QUEUE_SIZE);
        let (int_s, int_r) = mpsc::channel();

        Application {
            int_sender: Arc::new(Mutex::new(int_s)),
            int_recv: int_r,
            out_file: Arc::new(Mutex::new(DataFile::new(DATA_FILE_NAME))),
            data_queue: q,
            q_sender: Arc::new(Mutex::new(s)),
            q_receiver: Arc::new(Mutex::new(r)),
            consumers: Vec::new(),
            producers: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.init();

        self.init_int_handler();

        println!("Starting the process");

        self.start_workers();
        loop {
            sleep(WAIT_INTERVAL_MS);
            self.print_status();

            match self.int_recv.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn stop(&mut self) {
        println!("Stopping the application");
        self.stop_producers();
        println!("All {} producers have been stopped", self.producers.len());
        let queue_size = self.data_queue.len();
        if queue_size > 0 {
            println!("Data queue size is now {}", queue_size);
            println!("Waiting consumers to empty the queue");
        }
        self.data_queue.wait_until_empty();

        println!("Queue size is {} now", self.data_queue.len());

        self.out_file.lock().unwrap().flush();
        println!("Stopped")
    }

    fn init(&mut self) {
        let producers_count: usize = input_number("producer", MIN_PRODUCERS, MAX_PRODUCERS);
        let consumers_count: usize = input_number("consumer", MIN_CONSUMERS, MAX_CONSUMERS);

        self.create_producers(producers_count);
        self.create_consumers(consumers_count);

        self.out_file.lock().unwrap().open();
    }

    fn create_consumers(&mut self, count: usize) {
        for _ in 0..count {
            self.consumers.push(Worker::new());
        }
    }

    fn create_producers(&mut self, count: usize) {
        for _ in 0..count {
            self.producers.push(Worker::new());
        }
    }

    fn start_workers(&mut self) {
        for p in self.producers.iter_mut() {
            let s = self.q_sender.clone();

            p.start(move || {
                // TODO: create once
                let mut rng = rand::thread_rng();

                sleep(rng.gen_range(1..101));
                s.lock().unwrap().push(rng.gen_range(1..101));
            });
        }

        for c in self.consumers.iter_mut() {
            let r = self.q_receiver.clone();
            let file = self.out_file.clone();

            c.start(move || {
                // TODO: create once
                let mut rng = rand::thread_rng();

                sleep(rng.gen_range(1..101));
                let data = r.lock().unwrap().pop();
                file.lock().unwrap().write(data);
            });
        }
    }

    fn stop_producers(&mut self) {
        for p in self.producers.iter() {
            p.stop();
        }

        for p in self.producers.iter_mut() {
            p.wait()
        }
    }

    fn print_status(&self) {
        println!("Data queue size is: {}", self.data_queue.len());
    }

    fn init_int_handler(&self) {
        let app_int = self.int_sender.clone();

        ctrlc::set_handler(move || {
            println!("Stop gracefully");
            app_int.lock().unwrap().send(()).unwrap();
        }).expect("Unable to set Ctrl+C handler");
    }
}