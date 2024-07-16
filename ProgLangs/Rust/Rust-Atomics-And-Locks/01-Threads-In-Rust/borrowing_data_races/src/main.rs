use std::thread;
use std::sync::Arc;

//  immutable borrowing: 
// - borrowing something with &, this will give an immutable reference
// - reference can be copied and access is shared between all copies
// - compiler does not allow you to mutate though such reference

// mutable borrowing
// - borrow with &mut gives mutable reference
// - only active borrow of that data
// - mutating will not change any code that is current looking at it

// the borrowing rules prevent data races
// there may never be more than one mutable reference to an object
// - can only be broken with unsafe code


fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y = 10;
    } // mutable borrow of x ends here
    // it can be used again here

    // you can have multiple immutable borrows
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    // mutable and immutable borrows cannot coexist
    let mut s = String::from("no");
    let r1 = &s;
    //let  mut r2 = &mut s; // cannot borrow as mutable


    // example of a data race
    let counter = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            unsafe {
                // this is an exaple of a data rance since it is not atomic
                // we are using unsafe to get around the borrowing rules
                let counter_ptr = Arc::as_ptr(&counter) as *mut i32;
                *counter_ptr += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}
