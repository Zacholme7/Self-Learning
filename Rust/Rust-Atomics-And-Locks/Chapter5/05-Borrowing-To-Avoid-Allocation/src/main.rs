use Borrowing_To_Avoid_Allocation::Channel;
use std::thread;


fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split(); // get the sender and the receiver
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello world");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world");
    });
}



