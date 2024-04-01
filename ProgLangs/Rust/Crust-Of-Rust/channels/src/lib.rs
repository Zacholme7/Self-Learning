use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

/// Sender channel, used to send on the channel
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            inner: Arc::clone(&self.inner)
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);

        // need to notfiy any waiting receivers when it sends
        drop(queue); // drop the lock here
        self.inner.available.notify_one();
    }
}

/// Receiver channel, used to recieve on the channel
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    // if we end up here, wait for signal on the condvar
                    // thread goes to sleep and only wakes up if there is some reason for it to
                    // wake up
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

// inner structure of the sender and receiver
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

/// construct a new channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // construct a new inner
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::default(),
    };

    // wrap it in a arc and mutex
    let inner = Arc::new(inner);

    // make the sender and receiver
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}









































