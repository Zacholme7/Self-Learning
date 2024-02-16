// if we want to optimize for efficienty, we can trade some convenience for performance
// make the user responsible for shared channel object
// force the user to create a channel that can be borrowed by the Sender and Receiver
// sender and receiver borrow channel for a certain lifetime
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering::*;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>, // holds the message that we will pass around
    ready: AtomicBool, // signals if we have a message ready
}

// implement Sync trait
unsafe impl<T> Sync for Channel<T> where T: Send{}

// implement the Drop trait
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}


impl<T> Channel<T> {
    // move back to a new constructor
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    // split exclusive borrow into two share borrows
    // let both the sender and receiver reference the channel
    // exclusively borrows self through an exclusive reference but it splits that into two shared
    // references
    // 'a lifetime makes it clear that both of those objects borrow something with a limited lifetime
    // by overwriting *self, make sure it is in an expected state, also invoked drop on only self
    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (Sender { channel: self}, Receiver { channel: self})
    }
}

// Sender that borrows the channel
// need the lifetime to make sure the sender lives as long as the channel
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    // send a message
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) }; // write the message to the channel
        self.channel.ready.store(true, Release); // signal that we have a mesasge ready
    }

}

// Receiver that borrows the channel
// need the lifetime to make sure the receiver lives as long as the channel
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
    // check if there is a message ready
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    // recieve the message
    pub fn receive(self) -> T {
        // check if there is a message ready
        if !self.channel.ready.swap(false, Acquire) {
            panic!("No message available");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() } // read the message
    }

}


















