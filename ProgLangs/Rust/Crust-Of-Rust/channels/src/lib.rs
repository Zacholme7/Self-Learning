
use std::sync::{Arc, Condvar, Mutex};

/// Sender channel, used to send on the channel
pub struct Sender<T> {
    inner: Arc<Inner<T>>
}

/// Receiver channel, used to recieve on the channel
pub struct Receiver<T> {
    inner: Arc<Inner<T>>
}

// inner structure of the sender and receiver
struct Inner<T> {
    queue: Mutex<Vec<T>>
}

/// construct a new channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // construct a new inner
    let inner = Inner { queue: Mutex::default() };

    // wrap it in a arc and mutex
    let inner = Arc::new(inner);

    // make the sender and receiver
    (
        Sender {
            inner: inner.clone()
        },
        Receiver {
            inner: inner.clone()
        },
    )
}
