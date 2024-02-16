use std::sync::atomic::AtomicI32;

fn main() {
    // relaxed ordering does not provide any happens before relationship
    // do guarantee a total modification order of each individual atomic variable
    // all modifications of the same atomic variable happen in an order that is the same from the persepctive of every single thread

}

static X: AtomicI32 = AtomicI32::new(0);

// assume that a and b are concurrently executed by different threads

// only one thread modifies 5, so east to see that X: 0 -> 5 -> 15
// threads cannot observe any values from X that are inconsistent with the total modification order
// all threads will agree on a single order
fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a}, {b}, {c}, {d}");
}


// assuming threads modifying X, now two possible modification orders
// X: 0 -> 5 -> 15 or X: 0 -> 10 -> 15
// although, whatever happends first all threads observe the same order
// if one prints 10, then order must be 0 -> 10 -> 15 and none can print 5
fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}

