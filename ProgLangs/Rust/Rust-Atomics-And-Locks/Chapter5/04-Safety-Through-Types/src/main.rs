use Safety_Through_Types::channel;
use std::thread;

fn main() {
    // create a scoped thread
    thread::scope(|s| {
        let (sender, receiver) = channel(); // get the sender and receiver
        let t = thread::current();
        // spawn a new thread
        s.spawn(move || {
            sender.send("hello world"); // send a message on the channel
            t.unpark();
        });
        // while we have not got a message yet, park the current thread
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world");
    });
}

