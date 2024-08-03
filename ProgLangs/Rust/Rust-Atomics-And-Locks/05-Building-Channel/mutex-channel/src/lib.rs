use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

// channels can be used to send data between threads
// can use a VecDeque and just protect it with a mutex

// any send/recieve will block to lock the mutex
// if his vecdeque capacity, wait for one thread to finish reallocation
// can also grow without bounds

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}


impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        // get the lock
        self.queue.lock().unwrap().push_back(message);
        // notify if we have something waiting
        self.item_ready.notify_one();
    }

    pub fn recieve(&self) -> T {
        // lock the queue
        let mut lock = self.queue.lock().unwrap();
        loop {
            // try to recieve an item
            if let Some(item) = lock.pop_front() {
                return item;
            }
            // unlock the mutex while waiting to be notified
            lock = self.item_ready.wait(lock).unwrap();
        }
    }
}
