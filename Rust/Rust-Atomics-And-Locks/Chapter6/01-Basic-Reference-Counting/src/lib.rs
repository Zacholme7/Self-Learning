// first version will use AtomicUsize to count number of Arc objects that shrae an allocation
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::AtomicUsize;
use std::ops::Deref;
use std::sync::atomic::fence;


// not public, this is internal impl detail of our Arc
struct ArcData<T> {
    ref_count: AtomicUsize, // the amount of objs what share this
    data: T, // the data that this is holding
}

// Arc is effectively just a pointer to a shared ArcData object
pub struct Arc<T> {
    // represents a pointer ot T that is never null
    // compiler will assume it is not Send or Sync unless we tell it otherwise
    ptr: NonNull<ArcData<T>>,
}

// sending Arc<T> across threas results in T object being shared
// this reques T to be sync
unsafe impl<T: Send + Sync> Sync for Arc<T> {}
// Sending Arc<T> across threads can result in another thread dropping T
// which reqires T to be Send
unsafe impl<T: Send + Sync> Send for Arc<T> {}

// implementation details
impl <T> Arc<T> {
    // create new allocation of ArcData<T> with reference count of 1
    // Box::new to create new allocation
    // Box::leak to give up exclusive ownership of allocation
    // NonNuLL::from to turn it into a pointer
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    // We know the pointer will always point to valid ArcData<T> as long as Arc exists
    // Helper function to get from ARc to ArcData
    fn data(&self) -> &ArcData<T> {
        unsafe {self.ptr.as_ref() }
    }
}

// implement Deref trait to make Arc<T> transparently behave like a reference to T
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

// implement clone
// will use the same pointer after incrementing the reference counter
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // Todo: handle overflows
        // add one to the reference counter
        self.data().ref_count.fetch_add(1, Relaxed);
        Arc {
            ptr: self.ptr,
        }
    }
}

// implement Drop
// need to decrement reference count when dropping an Arc
// if the count is zero, then we need to drop and deallocate ArcData
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                // use Box::from_raw to reclaim exclusive ownership of the allocation
                // drop it right away using drop()
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    // Create two Arcs sharing an object containing a string
    // and a DetectDrop, to detect when it's dropped.
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    // Send x to another thread, and use it there.
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });

    // In parallel, y should still be usable here.
    assert_eq!(y.0, "hello");

    // Wait for the thread to finish.
    t.join().unwrap();

    // One Arc, x, should be dropped by now.
    // We still have y, so the object shouldn't have been dropped yet.
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    // Drop the remaining `Arc`.
    drop(y);

    // Now that `y` is dropped too,
    // the object should've been dropped.
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
}





