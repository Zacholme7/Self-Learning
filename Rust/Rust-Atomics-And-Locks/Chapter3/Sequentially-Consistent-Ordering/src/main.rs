use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::AtomicBool;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    // strongest memory ordering, Ordering::SeqCst
    // includes all guarantees of acquire ordering (for loads) and release ordering (for stores), and globally consistent order of operations
    // every single operation using SeqCst is part of a single total order that all threads agree on
    // almost never necessary in practice and regular aquire and release ordering sufficies

    // possible that neighter end up accessing S
    // impossible for both threads to access S and causeundefined behavior since SeqCst guarantees only one of them can win the race
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
