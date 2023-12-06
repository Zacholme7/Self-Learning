// Weak<T> also called a weak pointer does not prevent an object from getting dropped
// Weak<T> can exist without a T, but can be upgraded to Arc<T> with .upgrade()
// In Arc based strucutre, Weak can be used to break cycles
// we will use two counters, one for the number of things that reference T and another
// for the number of things that reference the ArcData<T>
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release};
use std::ptr::NonNull;
use std::sync::atomic::fence;

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

// implement clone for the arc
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { weak }
    }
}


// deref implementation
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get(); // get a pointer to the contents of the cell
        // unsafe code to promise it can safelty be shared at this point
        unsafe { (*ptr).as_ref().unwrap() }
    }
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

    // get a mutable refernce
    // have to check that there are no otehr Arc<T> or Weak<T> poitners beofre we give out a &mut T
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            // Safety: Nothing else can access the data, since
            // there's only one Arc, to which we have exclusive access,
            // and no Weak pointers.
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();
            // We know the data is still available since we
            // have an Arc to it, so this won't panic.
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }

    // move from arc from weak
    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

// dropping an Arc should decremetn both counters
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();
            unsafe { (*ptr) = None }
        }
    }
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

// import send and sync for weak
unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Weak<T> {
    // assume ptr always point at a valid ArcData<T>
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    // upgrade from weak to arc
    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }

            assert!(n < usize::MAX);
            if let Err(e) =
                self.data()
                    .data_ref_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc {weak: self.clone() });
        }
    }

}

// clone implementation
impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr : self.ptr }
    }
}


// dropping a weak should decrement its counter and drop and deallocat ArcData when the counter
// goes to zero
impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
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

    // Create an Arc with two weak pointers.
    let x = Arc::new(("hello", DetectDrop));
    let y = Arc::downgrade(&x);
    let z = Arc::downgrade(&x);

    let t = std::thread::spawn(move || {
        // Weak pointer should be upgradable at this point.
        let y = y.upgrade().unwrap();
        assert_eq!(y.0, "hello");
    });
    assert_eq!(x.0, "hello");
    t.join().unwrap();

    // The data shouldn't be dropped yet,
    // and the weak pointer should be upgradable.
    assert_eq!(NUM_DROPS.load(Relaxed), 0);
    assert!(z.upgrade().is_some());

    drop(x);

    // Now, the data should be dropped, and the
    // weak pointer should no longer be upgradable.
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
    assert!(z.upgrade().is_none());
}





























