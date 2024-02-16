// one shot channel: sending exactly one message from one thread to another
// pass a lot of responsibility over to the caller to use this safely
use std::mem::MaybeUninit; // bare bones unsafe version of Option<T>
use std::sync::AtomicBool;

// channel struct
pub struct Channel<T> {
    message: UnsafeCell<MaubeUninit<T>>,
    ready: AtomicBool,
}

// tell compiler that our channel is safe to share betwen threads
// or at least as long as T is send
unsafe impl<T> Sync for Channel<T> where T: Send {}

// implementation
impl<T> Channel<T> {

    // constructor
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()), // the message that the channel is sending
            ready: AtomicBool::new(false) // signals if there is a message
        }
    }

    // to send a message, need to store in cell and then release it to receieved
    // by setting ready flag to true
    // Safety: only call this once
    pub unsafe fn send(&self, message: T) {
        // get to obtain a pointer then dereference so that we can write into it
        (*self.message.get()).write(message); // store the message in the cell
        self.ready.store(true, Release); // signal that we have a message
    }

    // check is a message is ready
    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire);
    }

    // recieve a message
    // safety: only call this once
    // and only after is_ready() returns true
    pub unsafe fn receive(&self) -> T {
        // get method to obtain a pointer and then dereference so that we can access it
        // unsafely assume that it is initialized
        (*self.message.get()).assume_init_read()
    }
}



