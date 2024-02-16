use std::thread;
use std::time::Duration;

fn main() {
    // to create a new thread, call thread::spawn

    // the return type of thread::spanw is a JoinHandle
    // owned value that when we call join will wait for the thread to finish
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // rejoin with the main thread
    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // move in the closure takes ownership of the vector
    let handle = thread::spawn(move || {
        println!("here is a vector: {:?}", v);
    });
    handle.join().unwrap();


}
