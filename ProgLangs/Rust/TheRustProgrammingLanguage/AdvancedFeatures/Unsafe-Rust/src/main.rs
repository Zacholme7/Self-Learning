static mut COUNTER: u32 = 0;

fn main() {
    // is it better to reject valid programs than to accpet invalid ones
    // to switch to unsafe rust, use the unsafe keyword
    // can take 5 actions in unsafe rust
    // dereference raw pointer, call unsafe function/method, access/modify mutable static variable, implement unsafe trait, access field of union
    // doesnt turn off borrow check or disable any other safety checks

    // dereferenceing raw pointer
    // two new types called raw pointers
    // *const T and *mut T
    // allowd to ignore borrowing rules, arent guaranteed to point to valid memory, can be null, dont auto cleanup
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // have to use unsafe block to dereference the raw pointers
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    // calling an unsafe function or method
    unsafe {
        dangerous();
    }


    // accessing or modifying a mutable static variable
    unsafe {
        COUNTER += 1;
    }



}


unsafe fn dangerous() {}
