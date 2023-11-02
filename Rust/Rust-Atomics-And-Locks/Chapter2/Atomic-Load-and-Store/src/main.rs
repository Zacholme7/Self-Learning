use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::sync::atomic::AtomicU64;

fn main() {
    // load: loads the value stored in the atomic variable
    // store: stores a new value in it
    

    // example: using AtomicBool for a stop flag to inform other threads to stop running
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        // this thread will continue to run while we are still entering commands
        while !STOP.load(Relaxed) { // 
            println!("Doing some work");
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    STOP.store(true, Relaxed); // if stopped, set the stop flag to true so background_thread will stop
    background_thread.join().unwrap()

    // example: progress reporting
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn( || {
            for i in 0..100 {
                // do some work here
                num_done.store(i + 1, Relaxed); // increment the atomic to track the number of elements done
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Relaxed); // load the number of elements dont
            if n == 100 { break; }
            println!("Working... {n}/100 done");
            thread::park_timeout(Durationa::from_secs(1));
        }
    });
    println!("Done")


    // lazy initialization
    // have a value that we dont expect to change
    // can just have the first thread that needs it calculate it and then store it in atomic static so it is available to all threads
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}
