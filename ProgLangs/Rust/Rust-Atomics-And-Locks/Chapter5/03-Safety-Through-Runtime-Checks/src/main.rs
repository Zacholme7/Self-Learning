use Safety_Through_Runtime_Checks::Channel;
use std::thread;

fn main() {
    // Construct our channel
    let channel = Channel::new();
    // get the current thread
    let t = thread::current();
    // start a scoped thread that captures the scope
    thread::scope(|s| {
        // spawn a new thread to send a message on our channel
        s.spawn(|| {
            channel.send("hello world");
            // when we have send the message, unpark the thread
            t.unpark();
        });
        // while the channel is not ready, park the thread
        while !channel.is_ready() {
            thread::park();
        }
        // get the message from our channel
        assert_eq!(channel.receive(), "hello world");
    });
}

