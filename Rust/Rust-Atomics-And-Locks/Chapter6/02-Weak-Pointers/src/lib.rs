// Weak<T> also called a weak pointer does not prevent an object from getting dropped
// Weak<T> can exist without a T, but can be upgraded to Arc<T> with .upgrade()
// In Arc based strucutre, Weak can be used to break cycles
// we will use two counters, one for the number of things that reference T and another
// for the number of things that reference the ArcData<T>

use std::sync::atomic::AtomicUsize;
use std::cell::UnsafeCell;
use std::ptr::NonNull;

struct ArcData<T> {
    // The number of arcs
    data_ref_count: AtomicUsize,
    // number of arcs and weaks combined
    alloc_ref_count: AtomicUsize,
    // the data, none if there are no weak pointers left
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>,
}

impl<T> Arc<T> {
    // constructor for the Arc
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    alloc_ref_count: AtomicUsize::new(1),
                    data_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

// import send and sync for weak
unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}
