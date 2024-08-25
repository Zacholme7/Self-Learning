use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::fence;
use std::ops::Deref;
use std::sync::atomic::Ordering;

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

    // for mutation, can conditionally allow it 
    // only give out &mut T if ref counter is one, meaning no other Arc objs can be used to access the
    // same data
    // takes &mut Self so it can only be called like Arc::get_mut(&mut a)
    // returned mutable reference implicitly borrows the lifetime of the argument, meaning
    // nothing can use original arc as long as &mut T is around
    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            // nothing else can access this data since there is only one arc
            unsafe { Some(&mut arc.ptr.as_mut().data) }
        } else {
            None
        }
    }
}


// impl Deref to make Arc<T> behave like a ref to T
impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

// dont impl DerefMut since Arc<T> is shared ownership and we dont want it to provider &mut T

// we want to impl clone, will just inc a counter and use the smae pointer
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        self.data().ref_count.fetch_add(1, Ordering::Relaxed); // add one to the ref count
        Self {
            ptr: self.ptr
        }
    } 
}

// need to decrement counter when dropping, and deallocate if last one
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }

    }
}













































