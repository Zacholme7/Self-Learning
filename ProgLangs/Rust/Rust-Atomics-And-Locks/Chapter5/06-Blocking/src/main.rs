use Blocking::Channel;
use std::thread;

fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split(); // get the sender and the receiver
        s.spawn(move || {
            sender.send("hello world");
        });
        assert_eq!(receiver.receive(), "hello world");
    });
}



