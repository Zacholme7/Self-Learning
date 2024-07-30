// one shot channel, send one message from a thread to another

use std::mem::MaybeUninit;
// std::mem::MaybeUninit<T> -> essentialy bare bones unsafe version of Option<T>
// - have to keep track of whether it has been initialized or not

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool
}

// we need to tell compiler channel is safe to share between threads
// or at least as long as T is Send
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }


    // to send a message, need to store in cell, then release by setting ready
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Ordering::Release);
    }
}


