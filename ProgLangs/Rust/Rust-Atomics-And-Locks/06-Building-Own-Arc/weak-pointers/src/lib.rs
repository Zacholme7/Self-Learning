// - ref counting useful when repr strucutres consistening of multiple objects
// - Weak<T>, behaves like Arc<T> but does not prevent an object from getting dropped

// - A T can be shared between several Arc<T> and Weak<T> objects, when when all Arc are gone the T
// is dropped, regardless of it there are any weak ones left
// - Weak<T> can exist without a T, so cannot provider &T unconditionally
// - To access T given Weak<T>, must upgrade to Arc<T>  throught upgrade()
// return None if T has already been dropped

// - Weak<T> can be used to break cycles

// counter for the number of things that reference T and the number of things that ref the ArcData
// , count the Arc and Weak objects

use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;

struct ArcData<T> {
    // number of Arcs
    data_ref_count: AtomicUsize,
    // number of arcs and weaks combined
    alloc_ref_count: AtomicUsize,
    // the data, none if theres only weak poitners left
    data: UnsafeCell<Option<T>>,
}

// think of Weak<T> ad object resonsible for keeping ArcData<T> alive
pub struct Arc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Arc<T> {
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

impl<T> Weak<T> {
    // assume ptr field always points to valid ArcData<T>
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref().unwrap() }
    }
}
