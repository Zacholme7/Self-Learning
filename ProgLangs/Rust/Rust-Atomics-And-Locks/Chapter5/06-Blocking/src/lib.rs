// Dealing with the lack of a blocking interface for the channel
// to unpark the receiver, the sender needs to know which thread to unpark
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::Ordering::*;
use std::thread::Thread; // represents a handle to a thread and is what we need for calling unpark()
use std::thread;
use std::marker::PhantomData;

// channel structure
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
    // split method to get the sender and the receiver pairs
    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self, // this object as the channel
                receiving_thread: thread::current(), // the current thread that will allow us to bloc
            },
            Receiver {
                channel: self, // this object as the channel
                _no_send: PhantomData, // this will make it so that we cannot send it
            }
        )
    }
}

// sender for sending a message
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread
}

impl<T> Sender<'_, T> {
    // send a message along the channel
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) }; // send a message on the channel
        self.channel.ready.store(true, Release); // signal that we have a mesasge ready
        self.receiving_thread.unpark(); // unpark the thread so that we are no longer blocking
    }
}

// recieve to recieve a message
// make it more restrictive by not allowing it to be sent between threads
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>, // makes it so that it cannot be send since this does not implement send
}

impl<T> Receiver<'_, T> {
    // recieve a message
    pub fn receive(self) -> T {
        // while we do not have a message ready, park the thread
        while !self.channel.ready.swap(false, Acquire) {
            thread::park(); 
        }
        unsafe { (*self.channel.message.get()).assume_init_read() } // recieve the message
    }
}



