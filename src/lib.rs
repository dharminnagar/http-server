use std::{sync::{Arc, Mutex, mpsc}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(limit: usize) -> ThreadPool {
        assert!(limit > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(limit);

        for id in 0..limit {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }


        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {

                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("worker {id} got a job; executing...");

                        job();
                    },
                    Err(_) => {
                        println!("worker {id} shutting down");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers.drain(..) {
            println!("shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}