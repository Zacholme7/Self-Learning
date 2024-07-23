use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

// sequentially consistent

fn main() {
    // the stongest of all the orderings
    // includes guarantees of acquire ordering, release ordering, and also guarantees a globally
    // consistent order of operations

    // operation that uses this is part of single total order that all threads agree upon
    // almost never necessary in practice, acquire release usually is sufficie
    //
    // SeqCst Store + Acquire Load: Forms a happens-before relationship
    // Release Store + SeqCst Load: Also forms a happens-before relationship
    // SeqCst Store + SeqCst Load: Forms an even stronger relationship

    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('!') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
