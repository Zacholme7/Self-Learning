use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // for message sending concurrency, rust has channels
    // channel has two halves: transmitter and receiver
    // channel is closed if the trasmitter or receiver is dropped

    // create multiple produce single consumer channel
    // tuple of sender and receiver
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // make another transmitter

    // send messages in a thread with one transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        // send a message with the transmitter
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // val is no longer valid here since the ownership has been passed
    });

    // send messages in another thread with the other transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("vals"),
            String::from("for"),
            String::from("you"),
        ];
        // send a message with the transmitter
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // val is no longer valid here since the ownership has been passed
    });

    // recieve the message
    // will block the threads execution until a value is send
    // also try_recv which does not block 
    for received in rx {
        println!("Got {}", received);
    }
}
