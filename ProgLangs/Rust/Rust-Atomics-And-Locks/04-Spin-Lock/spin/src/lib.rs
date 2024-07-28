use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

// unsafecell can give us raw pointer to contentes through get()


// need to tie unlocking operation to the end of the &mut T
// wrap in type that behavies like ref, but implements drop
// lifetime of the guard is tired to the spinlock
// - guard cnanot outlive the lock
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}


// need guard to behavior like an exclusive reference

// spin lock
// attempting to lock already locked will make the current thread just "spin" until it can get the lock
// can be wasteful of cycles depneding on the contention/holding time

pub struct SpinLock<T> { // have to be generic over what we are storing
    locked: AtomicBool,
    value: UnsafeCell<T>, // interior mutability since we are sharing the lock but want to modify data
}

// if T is send, we implement sync for the lock since UnsafeCell is not 
// promise that it is actually safe for T to shared between threads
// T does not need to be sync since only one thread can access it at a time
unsafe impl<T> Sync for SpinLock<T> where T: Send {}


impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { 
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value)
        }
    }

    pub fn lock(&self) -> Guard<T>{
        // all changes after this will be visible
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        // existantce of guard means that the lock is held
        Guard { lock :self }
    }

    pub fn unlock(&self) {
        // all changes before this will be visible
        self.locked.store(false, Ordering::Release);
    }
}

    