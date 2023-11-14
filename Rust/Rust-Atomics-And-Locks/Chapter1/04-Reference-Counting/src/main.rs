use std::rc::Rc;
use std::thread;
use std::sync::Arc;

fn main() {
    // we can share ownershp
    // keep track of number of owners and drop when there are no owners left
    // provided through std::rc::Rc
    // cloning it will not allocate anything new, but increment a counter stored next to the contained vlaue
    // they share ownership

    // a and b both own and point to the same value
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());

    // Rc is not thread safe
    // thread::spawn(move || dbg!(b));  this throws an error
    // use std::sync::Arc instead
    // stands for atomically reference counted
    // guarantees that modifications to reference counter are atomic

    let a = Arc::new([1, 2, 3]); // reference count is 1
    let b = a.clone(); // reference count is 2

    // both threads get their own arc which points to the same data
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));

    // trick: to prevent naming new clones, just open closure in the thread and shadow the variable
    let a = Arc::new([1, 2, 3]);
    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    // Rc<T> and Arc<T> do not give you mutable access to their contained values
}
