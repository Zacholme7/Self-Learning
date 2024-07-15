use std::thread;

fn main() {
    // sometimes we know for sure that a thread will not outlive a certain scope
    // in this case, we also know that it can safely borrow things if they outlive the scope
    // 'std::thread::scope' will spawn scoped thread,
    //  - spawn thread that cannot outlive the scope of the closure that we pass to it
    let numbers = vec![1, 2, 3];
    // captures s, which represents the scope, which is a Scope object
    thread::scope(|s| {
        // none of the threads spawned here can outlive the scope
        // because of this, dont need 'static lifetime bound
        s.spawn(||{
            println!("Length: {}", numbers.len());
        });
        s.spawn(||{
            for n in &numbers {
                println!("{n}");
            }
        });

    });
    println!("Hello, world!");
}
