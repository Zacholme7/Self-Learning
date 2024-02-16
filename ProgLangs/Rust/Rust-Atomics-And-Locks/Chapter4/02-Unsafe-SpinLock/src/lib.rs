use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;

// want exclusive reference (&mut T) to the data protected by the lock
// need to use interior mutability
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// UnsafeCell does not implement sync, so we need to implement it
// for all T that implement send
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    // constructor that takes in a T
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // lock function that will now return &mut T
    // makes it so that the user does not have to write unsafe code, we do it here instead
    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    // cannot write a lock function
    // Safety: The &mut T from lock() must be gone!
    // (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }

}

