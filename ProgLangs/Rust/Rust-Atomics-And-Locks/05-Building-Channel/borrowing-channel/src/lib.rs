// can also make users responsible for shared channel object
// can make use create channel that can be borrowed from sender and receiver

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

// now we are tying the sender and receiver to the channel
// they cannot outlive the lifetime of the channel
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<t> channel<t> {
    pub const fn new() -> self {
        self {
            message: unsafecell::new(maybeuninit::uninit()),
            ready: atomicbool::new(false),
        }
    }

    // exclusivley borrows self through an exclusive reference
    // but splits it into two shared references
    // caller cannot move or borrow channel as long as sender and receiver exists
    pub fn split<'a>(&'a mut self) -> (sender<'a, t>, receiver<'a, t>) {
        *self = self::new(); // reassign self to a new channel
        (sender { channel: self }, receiver { channel: self })
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
