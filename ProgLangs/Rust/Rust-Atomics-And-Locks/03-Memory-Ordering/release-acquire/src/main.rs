use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::thread;

static DATA: AtomicUsize = AtomicUsize::new(0); // static mut DATA: usize = 0;
static READY: AtomicBool = AtomicBool::new(false);

static mut DATA2: String =  String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn main() {
    // relase and acquire ordering used to form happens before relationship between threads
    // release applies to store operations, acquire applies to load operations
    // aquire load operations observes the results of a release store operation

    // when using fetch-and-modify/compare-exchange
    // aquire only applies to the load and release only applies to the store
    // AcqRel used to represent the combination

    // when thread done storing data, relase-acquire is used to form happens before relationship
    // everything that happens before Release-store is visible after acquire load
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed); // unsafe {DATA = 123}
        READY.store(true, Ordering::Release); // everything before this store...
    });

    while !READY.load(Ordering::Acquire) { // .. is visible after this loads true
        thread::sleep(std::time::Duration::from_millis(100));
        println!("waiting");
    }

    println!("{}", DATA.load(Ordering::Relaxed)); // only possible value is 123
    // if Release ordering was used, main thread may have saw true in ready before 123 was stored
    // dont technically need atomics for data since we know ordering, but compiler doesnt like it

    // mutexes are most common use case for release acquire ordering
    // check if unlocked using acquire and set back to unlocked using release
    // happens before between unlocking and locking

    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });

                            

}


fn f() {
    // takes two memory ordering arguments
    // 1) for case when comparison succeeded and store happens

    if LOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
        // we hold the exclusive lock, so nothing else is accessing DATA
        unsafe { DATA2.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}
