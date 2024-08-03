// one shot channel, send one message from a thread to another

use std::mem::MaybeUninit;
// std::mem::MaybeUninit<T> -> essentialy bare bones unsafe version of Option<T>
// - have to keep track of whether it has been initialized or not

use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::task::ready;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

// we need to tell compiler channel is safe to share between threads
// or at least as long as T is Send
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    // to send a message, need to store in cell, then release by setting ready
    pub fn send(&self, message: T) {
        // return the old value and set in_use to true, if old value is true, panic since
        // this is already in use. set value to true if old value is false since we are now
        // sending
        // can use ordering::relaxed since it is only used to prevent
        // concurrent send operations, we dont need any synchronized access
        if self.in_use.swap(true, Ordering::Relaxed) {
            panic!("we are already sending")
        }
        unsafe {
            (*self.message.get()).write(message);
        }

        // initialization of the message will be finished from perspective of thead if loads true
        // from ready with acquire ordering
        self.ready.store(true, Ordering::Release); // "release" message to the receiver
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn recieve(&self) -> T {
        // sets ready to false and returns its old value
        // if ready is true, return true and set to false, !true, will then recieve message
        // if ready is false, return false and set to false, will panic since no message
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no value");
        }

        // unsafely assumes already init and isnt being used to preoduce multiple copies of non
        // copy objects
        unsafe { (*self.message.get()).assume_init_read() }
    }
}


// dont need an atomic operations because an object can only be dropped
// if it is fully owned by whatever thread is dropping with no outstanding borrows.
// can use get_mut() to prove this
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}


















