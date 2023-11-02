use std::sync::atomic::AtomicBool;

// spinlock struct
pub struct SpinLock {
    locked: AtomicBook, // atomicbool to represent if it is locked or not
}

impl SpinLock {
    // constructor function
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false)
        }
    }

    // lock method
    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop(); // tells processor that we are spinning while waiting for something to change
        }
    }

    // unlock method
    pub fn unlock(&self) {
        self.locked.store(false, Release); // set locked to false to indicate that no one is holding the lock
    }
}