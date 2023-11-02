use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
    // message passing is one form of concurrency
    // another is having multiple threads accessing same shared data
    // shared mmemory concurrency is like multiple ownership

    // mutex allows only one thread to access some data at any given time
    // to access data in mutex, thread must acquire lock
    // must acquire lock before using the data
    // when youre done with the mutex, you must unlock it, automatically handleled in rust

    let m = Mutex::new(5);
    
    {
        // lock() returns MutexGuard, implements Drop which released lock automatically when out of scope
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}",m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
