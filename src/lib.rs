
use std::thread;
pub struct ThreadPool;

impl ThreadPool {
    pub fn new (thread_count : usize) -> ThreadPool {
        
        assert!(thread_count > 0);
        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            workers.push(Worker::new(id));

        }

        ThreadPool {workers};
    }
    pub fn execute <F> (&self, f: F)
    where F:FnOnce() + Send + 'static, 
    {

    }
    
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,

}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id: id, thread: thread }
    }
}