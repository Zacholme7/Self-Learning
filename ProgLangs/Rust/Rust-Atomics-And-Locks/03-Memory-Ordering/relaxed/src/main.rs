use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // atomic operating using relaxed ordering do not provider any happens-before relationship
    // They do guarantee a total modification order
    // - "all modification of the same atomic var happen in an order that is the same from the
    // perspective of every single thread"


    println!("Hello, world!");
}



// only one thread modifies X
// one possible order. X->0->5->15
fn a() {
    // this is a single variable, so the threads must agree on the order
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}


// "0 0 0 0", "0 0 5 15" and "0 15 15 15" are all possible, maintain the order
// "0 5 0 15" or "0 0 10 15" are not possible, do not maintain order

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}


// now 2 possible modification orders
// 0 -> 5 -> 15 or 0 -> 10 -> 15
// whatever happens first, all threads observe the same order
fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}
