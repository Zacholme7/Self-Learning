use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

static X: AtomicUsize = AtomicUsize::new(0);
static Y: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // mem model defines order in whic operations happen in terms of
    // happens before relationships

    // abstract model that is not concerened with low level machine details
    // only defines situations where one thing is guaranteed to happen before another

    // basic happens before rule
    // - everything that happens within the same thread happens in order
    // - if thread executing f(); g(), then f() 'happens before' g()

    // betweent threads, only occur in few cases such as spanwing/joining threads,
    // locking/unlocking mutex

    // Relaxed memory ording (Ordering::Relaxed)
    // - most basic and performant
    // - by itself, never results in cross thread happens before relationships
    // - does not provider guarantees for ordering between threads
    // - at its core, it only guarantees atomicity
    

    // imagine each atomic operation with relaxed ordering as a independent messenger
    // - each messanger (atomic op) carries message (value) to mailbox (mem location)
    // - messangers work independently and dont coordinate, 
    

    // spawning thread crates happens before between what happened before spawn()..
    // .. and the new thread
    // same for join,, happens before between join() call and what happends after

    X.store(1, Ordering::Relaxed); // we know this happens before..
    let t = thread::spawn(f); // ..the load in X
    X.store(2, Ordering::Relaxed); // this is unpredicatble store here or load in f may
                                               // run first
    t.join().unwrap(); // this join happens before..
    X.store(3, Ordering::Relaxed); // .. the store to 3 hereh

    println!("Hello, world!");
}

// reminder, within one thread everything happens in order
// 1 happens beore 2 and 3 happens before 4 (within same thread, above)
// understand that 3 loading 10 does not result in a happens before relationships
// ... with value getting stored in 2

fn a() {
    X.store(10, Ordering::Relaxed); // 1
    Y.store(20, Ordering::Relaxed); // 2
}

fn b() {
    let y = Y.load(Ordering::Relaxed); // 3
    let x = X.load(Ordering::Relaxed); // 4
    println!("{x}, {y}");
}


fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}
