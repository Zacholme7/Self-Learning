use std::thread;
use std::cell::Cell; // can only be used with copy types
use std::cell::RefCell; // can be used with any type
// moves borrowing rules from compile time to runtime

// a data type with interior mutability slighly bends the borrowing rules
// - can allow mutation through an immutable reference

// when interioir mutability involved, shared reference (&) and exclusive reference (&mut ) more accurate
// methods that logically dont modify the object but need to update some internal state
// runtime checks rather than compile time checks
fn main() {
    // 'std::cell::Cell<T>' wraps a T but allows mutation through a shared reference
    // allows you to copy the value out (if T is copy) or reaply it as a whole
    // single thread 
    // when you need to mutate data that is immutable, want to share mutable ref without
    // using refcell or mutex, simple ops where atomics are not necessary
    let a = Cell::new(10);
    f(&a, &a);

    // demonstration of interior mutability via Cell
    let counter = Counter::new(10);
    println!("Initial value: {}", counter.get());
    let ref1 = &counter; ref1.increment();
    let ref2 = &counter; ref2.increment();
    println!("After increments: {}", counter.get());


    // 'std::cell::RefCell' does allow you to borrow its contents, not just copy
    // holds T and a counter that keeps track of all borrrows
    // if you try to borrow while already mutably borrowed, it will panci
    // when you need interior mutability for non copy types
    // often used with Rc
    let numbers = RefCell::new(vec![1, 2, 3]);
    numbers.borrow_mut().push(1); // mutable borrow to T


    // RwLock, reader-writer lock is a concurrenct version of RefCell
    // holds T and tracks outstanding borrows
    // does not panic on conflicting borrows, blocks current thread and waits for other to dissapear
    // often used with Arc
    // more performance than mutex when you have a lot of reads
    // read() and write()

    // UnsafeCell<T> is the primitive building block for interior mutability
    // get() method that provides a pointer to the underlying
    // often wrapped in another type that provides safety

    println!("Hello, world!");
}


// although A and B are immutable references to the same thing
// the value can change still via the interior mutability of cell
fn f(a: &Cell<i32>, b: &Cell<i32>) {
        let before = a.get(); // retrieve the value from the cell
        b.set(b.get() + 1); // set the value of b
        let after = a.get();
        if before != after {
                // this is now possible
        }
}

// this is a demonstartion to above to elaborate on how it is doing that wihtout 
// a mutable reference, we cannot even call this because we cannot have a mutable
// reference and an immutable one at the same time
fn b(a: &i32, b: &mut i32) {
        let before = *a;
        *b +=1;
        let after = *a;
        if before != after {
                // this will never happen
        }
}


// example of interior mutability with cell
struct Counter {
        value: Cell<i32> // so we can have interioir mutability
}

impl Counter {
        fn new(initial: i32) -> Self {
                Self {value : Cell::new(initial) }
        }
        // notice we have a &self which means we should not be able to modify it
        // yet we are still able to, this is due to interior  mutability
        fn increment(&self)  {
                let current = self.value.get(); // get the value out
                self.value.set(current + 1);
        }
        fn get(&self) -> i32 {
                self.value.get()
        }
}



























