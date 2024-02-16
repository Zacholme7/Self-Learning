use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

struct SpinLock {
    flag: AtomicBool,
}

impl SpinLock {
    // Create a new spinlock
    fn new() -> SpinLock {
        Self {
            flag: AtomicBool::new(false),
        }
    }

    // Lock the spinlock
    fn lock(&self) {
        while self.flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            thread::yield_now();
        }
    }

    // Unlock the spinlock
    fn unlock(&self) {
        self.flag.store(false, Ordering::SeqCst);
    }
}

fn work_on_something(lock: &SpinLock) {
    println!("{:?} is trying to lock", thread::current().id());
    lock.lock();
    println!("{:?} acquired the lock", thread::current().id());

    thread::sleep(Duration::from_millis(100));

    println!("{:?} is unlocking", thread::current().id());
    lock.unlock();
    println!("{:?} unlocked the lock", thread::current().id());
}

fn main() {

    let lock = SpinLock::new();
    let lock_arc = std::sync::Arc::new(lock);


    let lock_clone1 = lock_arc.clone();
    let t1 = thread::spawn(move || work_on_something(&lock_clone1));
    let lock_clone2 = lock_arc.clone();
    let t2 = thread::spawn(move || work_on_something(&lock_clone2));

    let _ = t1.join();
    let _ = t2.join();
}
