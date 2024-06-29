
use std::{thread, sync::mpsc, sync::Arc, sync::Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    pub worker: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!( size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut worker = Vec::with_capacity(size);

        for id in 0..size {
            worker.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool{worker, sender}
    }

    pub fn excute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize, 
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });

        Worker { id, thread }
    }
}
