use std::mem::MaybeUninit; // bare bones unsafe versio of Option<T>
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed};

// one shot channel: sending exactly one message from one thread to another

// structure definition
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>, // the message that is being passed
    ready: AtomicBool, // is a message ready
    in_use: AtomicBool, // is there a message currently being send
}

// need to tell compiler it is safe to share between threads as long as T is send
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    // constructor
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    // to send a message
    // need to store in cell and set ready flag to true
    // only can call this once right now
    pub fn send(&self, message: T) {
        // see if a message is already being sent
        if self.in_use.swap(true, Relaxed) {
            panic!("cant send more than one message");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    // check is a message is ready
    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    // panics if no message is available yet
    // receives a message if there is one
    pub  fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}


// implementing the drop trait
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}




