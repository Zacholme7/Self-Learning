use std::sync::Arc;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::sync::atomic::Ordering;

// to prevent function from being called more than once, take by value to consume the object
// take a sender and receiver to make sure we cannot end up with multiple copies of them


struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

// implement sync on the condition that T is send
unsafe impl<T> Sync for Channel<T> where T: Send {}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false)
    });
    (Sender { channel: a.clone() }, Receiver { channel: a.clone() } )
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>
}

impl<T> Sender<T> {
    // notice, this takes self, this means that this can only be called once!!
    // we are guaranteeing this via the type system
    pub fn send(self, message: T) {
        // store the value in the message cell
        unsafe { (*self.channel.message.get()).write(message); }
        // signal that a value is ready, release it 
        self.channel.ready.store(true, Ordering::Release);
    }
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>
}

impl<T> Receiver<T> {
    // can take ref since just checking if ready
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    // again, take self to make sure it can only be called once
    pub fn receive(self) -> T {
        // swap, return the old value and store false
        if !self.channel.ready.swap(false, Ordering::Acquire) {
            panic!("We do not have a message ready");
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

fn main() {
    thread::scope(|s| {
        let (sender, receiver) = channel();
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world!");
    });
}
