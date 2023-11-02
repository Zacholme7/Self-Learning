use std::sync::atomic::AtomicI32;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn main() {
    // memory model defines order in whici operations happen in terms of happens before

    // Relaxed memory ordering is the most basic and never results in any cross thread happens before relationships

    // spawning a thread crates a happens before relationship between what happened before the spawn() call
    // and the new thread
    // joining creates a happens before relationship between the joined thread and what happens after the join() call
    // example: the following assertion cannot fail
    X.store(1, Relaxed);
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    assert!(x == 1 || x == 2);
}




fn a() {
    // since we used relaxed ordering, there is no happens before ordering
    X.store(10, Relaxed); 
    Y.store(20, Relaxed);
}

fn b() {
    // since we used relaxed ordering, there is no happens before ordering
    let y = Y.load(Relaxed); 
    let x = X.load(Relaxed); 
    println!("{} {}", x, y);
}
