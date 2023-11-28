// we can add some checks to make misue result in a panic with a cleaner message
use std::mem::MaybeUninit; // bare bones unsafe version of Option<T>
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::*;

// channel struct
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool, // indicate whether the channel has been taken in use
}

// tell compiler that our channel is safe to share betwen threads
// or at least as long as T is send
unsafe impl<T> Sync for Channel<T> where T: Send {}


// makes sure that a message that is never receieved is dropped
// implement drop for the channel
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

// implementation
impl<T> Channel<T> {
    // constructor
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()), // the message that the channel is sending
            ready: AtomicBool::new(false), // signals if there is a message
            in_use: AtomicBool::new(false), // signal if the channel is in use
        }
    }

    // to send a message, need to store in cell and then release it to receieved
    // by setting ready flag to true
    // panics when trying to send more than one message
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("cant send more than one message")
        }

        // get to obtain a pointer then dereference so that we can write into it
        unsafe { (*self.message.get()).write(message) }; // store the message in the cell
        self.ready.store(true, Release); // signal that we have a message
    }

    // check is a message is ready
    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    // recieve a message
    // panics if no message is available yet or is was already consumed
    pub fn receive(&self) -> T {
        // check to see if a message is ready
        if !self.ready.swap(false, Acquire) {
            panic!("No Message Available")
        }
        // we know a message is ready
        // get method to obtain a pointer and then dereference so that we can access it
        // unsafely assume that it is initialized
        unsafe { (*self.message.get()).assume_init_read() }
    }
}









