fn main() {
    // a data type with interior mutability slighly bends the borrowing rules
    // the types can allow mutation through an immutable reference
    // when interior mutable types are invovled, shared and exlusive are better terms
    // a shared reference (&T) can be copied and shared with others
    // a exclusive reference (&mut T) guarantees its the only exlusive borrowing of that T
}


// CELL
// allows mutation through a shared reference
// to avoid undefined behavior, only allows you to copy the value out or replace it with another
// can only be used in a single thread

// it is possible for the if condition to be true
// both a and b might refer to the same value and mutating through b might affect a as well
// still may assume only one thread is accessing the cell 
use std::cell::Cell<T>;
fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("the are not the same");
    }
}

// sometimes not always east to work with
// cant directly let us borrow the value it holds
// to mutate
// need to move value out, modify it, then put it back 
fn e(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); // replaces contents of cell with empty vec
    v2.push(1);
    v.set(v2); // sets the modified vec
}

// REFCELL
//std::cell::RefCell does allow you to borrow its contents as a small runtime cost 
// holds a T but also holds a counter that keeps track of outstanding borrows
// if you try to borrow while mutably borrowed, or vise versa, it will panic
// can only be used in a single thread
// borrowing is done by calling borrow or borrow_mut
use std::cell::RefCell;
fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // we can modify vec directly
}
// these two are useless when we need to do something with multiple threads

// MUTEX and RWLOCK
// a RwLock, reader writer lock, is the concurrnet version of RefCell
// RwLock<T> hols a T and tracks any outstanding borrows
// it does not panic on conflicting borrows, it just blocks the current thread (putting it to sleep)
// while waiting for conflicting borrows to dissapear
// borrowing the contents of RwLock is called locking
// by locking it, we block concurrent conflicting borrows which prevents data races
// Mutex is similar, it only allows exclusive borrows

// ATOMICS
// aomtic types represent the concurrent version of a cell
// avoid undefined behavior by making us copy values in and out as a while without borrowing contents directly
// cannot be arbitrary size, so no atomic<T> type

// UNSAFE CELL
// UnsafeCell is the primitive building block for interior mutability
// UnsafeCell<T> wraps a T but does not come with any conditions or restrictions
// get() just gives a raw pointer which can only be used in unsafe bloc
// not used directly but wrapped in another type to provide safety


