use std::sync::atomic::Ordering::{Acquire, Release};
use std::time::Duration;
ust std::sync::atomic::{AtomicU64, AtomicPtr, AtomicBool};

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    // release and acquire memory ordering are used in a pair to form a happens before relationship between threads
    // Release memory ordering applies to store operations
    // acquire memory ordering applies to load operations

    // happens before is formed when acquire-load op observes the reqult of a relase store op
    // the store and everything before it happens before the load and everything after it

    // when using Acquire for fetch-and-modify or compare-and-exchange, applies only to part of op that loads the values
    // and release only applies to the store part of the operation
    // AcqRel is used to represent combination of acquire and released

    // happens before relationship is formed betwen the release-store and the acquire load
    // we know for sure that happened before the release store to READY is visible to everything that happens after the acquire load
    // one thread "releases" data by atomically storing and another thread "acquires" data by atomically loading
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // everything from before this store ... 
    });

    while !READY.load(Acquire) { //.. is visible after this loads true
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));

    // mutexes are the most common case for release and acquire ordering
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}

// demonstration of mutex locking pattern
static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relased).is_ok() {
        unsafe { DATA.push('!')};
        LOCKED.store(false, Release);
    }
}

// example: lazy initialization with indirection
// AtomicPtr<T> is atomic version of a *mut T: a pointer to T
// null pointer as placeholder for initial state, use compare and exchange to replace with a valid pointer
// impelemntation for some arbirary type Data
fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe {Box::from_raw(p)});
            p = e
        }
    }

    unsafe {&*p}
}


