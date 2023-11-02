// channels can be used to send data between threads
// simple channel implementation just used a VecDeque
// protect it with a mutex and use it as a queue of data
// use CondVar to make it so that it is blocking

// Channel Structure
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>, // holds the messages
    item_ready: Condvar, // signals when there is a message ready
}

impl<T> Channel<T> {
    // constructor
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    // send a message
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message); // lock the mutex and add message
        self.item_ready.notify_one(); // send noti that item is read
    }

    // receive a message
    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap(); // lock and obtain the queue
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();// wait if there is no message ready yet
        }
    }
}