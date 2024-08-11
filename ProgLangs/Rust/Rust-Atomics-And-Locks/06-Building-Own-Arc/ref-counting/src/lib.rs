use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::ops::Deref;

// a cloned Arc will share the original allocation without creating a new one

// not public, it is an internal implementation detail
struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

// cannot make Box<ArcData<T>>, Box represents exclusive ownership
// cant use reference

// have to use pointer to handle allocation and ownership manually
// std::ptr::NonNull<T> which repr pointer to T that is never null
// - this way Option<Arc<T>> will be same size as Arc<T>
// - null pointer repr null
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

// when using box, compiler automatically knows for which T it should make strut send/sync
// with raw pointer or NonNull, it does not assume unless we tell it

// Sending Arc<T> across threads results in T being shared, T needs to be sync
// another thread could drop T which means T needs to be send
// * Arc<T> should be Send iff T is both Send and Sync
// * same goes for sync

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

// Box::new to create new allocation, Box::leak to give up exclusive ownership, NonNull::from to
// turn into ptr
impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    // pointer will always point to valid ArcData<T> as long as Arc object exists
    // compiler does not check this for us
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}



// impl Deref to make Arc<T> behave like a ref to T
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}











































