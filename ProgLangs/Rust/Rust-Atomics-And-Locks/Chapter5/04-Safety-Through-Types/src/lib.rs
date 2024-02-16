// channel is represented by a pair of sender and receiver
// can create a channel by calling channel()
//  give them sone sender and one receiver that can be passed around
//  cannot be multiple copies of them though
//  send and recieve take self by value to make sure they can only be called once
use std::mem::MaybeUninit; // bare bones unsafe version of Option<T>
use std::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::*;
use std::sync::Arc;

// The channel
struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>, // The message that is being sent
    ready: AtomicBool, // signals if the message is ready
}

// implement sync for the channel
unsafe impl<T> Sync for Channel<T> where T: Send {}

// create a channel and a sender-receiver pair 
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (Sender { channel: a.clone() }, Receiver { channel: a})
}

// implement drop
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

// Sends the message
pub struct Sender<T> {
    channel: Arc<Channel<T>>
}

// Recieves the message
pub struct Receiver<T> {
    channel: Arc<Channel<T>>
}

impl<T> Sender<T> {
    // implement sending on the channel
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) }; // write a message to the channel, send it
        self.channel.ready.store(true, Release); // signal that we have a message ready
    }
}

impl<T> Receiver<T> {
    // see if we have a mesasge on the channel
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed) // check if there is message to be recieved
    }

    // recieve a message on the channel
    pub fn receive(self) -> T {
        // check if we have a message ready
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available")
        }
        unsafe { (*self.channel.message.get()).assume_init_read() } // recieve the message
    }
}

