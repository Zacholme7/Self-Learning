use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Acquire;
// to provide fully safe interface, need to tie unlocking operation to the end of the &mut T
// wrap reference in own type that behaves like a ref, but implements drop trait
// type is often called a guard, guards the state of the lock and responsible until it is dropped

// contain ref to SpinLock so it can both access UnsafeCell and reset AtomicBool
// lifetimes to make sure that guard cannot outlive spinlock
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// to make Guard<T> behave like exclusive reference, have to implement Deref and DerefMut
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

// allows us to remove unsafe unlock method
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}


unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
  
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // now return a guard that contains the spinlock
    // this is the only way that a user can obtain a guard
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard {lock : self }
    }

}
