use std::thread;

fn main() {
    // if we know that thread will not outlive certain scope
    // it can safely borrow things that do not live forever
    
    // thread::scope spawns scoped threads
    // cannot outlive the scope of the closure that is passed
    let numbers = vec![1, 2, 3];
    thread::scope(|s| { // argument s representing the scope
        // use s to spawn threads
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    }); // at end, all threads that have not been joined are auto joined
}
