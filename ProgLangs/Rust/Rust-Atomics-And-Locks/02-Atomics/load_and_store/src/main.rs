use std::sync::atomic::AtomicBool; // atomic boolean
use std::sync::atomic::Ordering::Relaxed; // this is the simplest ordering
use std::thread;
// atomic operations allow different threads to safely read and modify same variables
// main building block for anything with multiple threads
// 'std::sync::Atomic'

// every atomic operation takes argument of type 'std::sync::atomic::Ordering'
// - determines what guarantees we get about relative ordering of operations


fn main() {
    // atomic load and store operations
    // load: atomically loads the value stored in the atomic variable
    // store: atomically stores a new value in the atomic variable
    //  - takes a shared reference even though it modifies it

    static STOP: AtomicBool = AtomicBool::new(false);
    // spawn a thread to do the work
    let background_thread = thread::spawn(|| {
        // load in the atomic bool stop flag, this will happen in 'one step'
        while !STOP.load(Relaxed) {
            // simulate some work
            println!("In the background just doing my thing");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // main thread will do something
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    // atomically store true in the STOP flag, this will make the thread break out of its loop
    // so we can join on it right after
    STOP.store(true, Relaxed);
    background_thread.join().unwrap();

}
