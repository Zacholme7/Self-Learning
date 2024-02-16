fn main() {
    // rust checks at compile time rather than runtime
    // goal of rust is to ensure there is never undefined behavior
    // also want to prevent undefined behavior at compile time instead of runtime
    let x = true;
    read(x);

    // variables live in frames, a mapping from variable to values in a single scope
    // frames are organized into a stack of currently called functions
    // after fucntion returns, rust deallocates frames 
    let n = 5; // frame holds n = 5
    let y = plus_one(n); // frame holds n = 5 & y = 6
    println!("The value of y is {}", y);

    // when reading variable, value copied from slot in stack frame
    let a = 5; 
    let mut b = a; // value of a is copied into b
    b += 1; // b is independent copy from a

    // to transfer access to data without copying it, rust uses pointers
    // pointer is a value that describes a location in memory, aka the heap
    // Box is used to put data onto the heap
    let a = Box::new([0, 1_000_000]);
    let b = a; // pointer in a is MOVED into b

    // rust does not allow for manual memory managmenet
    // box deallocation: if a variable owns a box, when rust deallocates the variables frame, then rust deallocates the box's heap memory

    // collections such as vec, string, and hashmap use boxed to hold a variable number of elements
    let first = String::from("hello");
    let full = add_suffix(first);
    println!("{full}");

    // variables cannot be used after being moved. 
    // println!("{full} originally {first}"); throws an error since first was moved into the function call

    // Cloning avoids moves

}

fn add_suffix(mut name: String) -> String {
    name.push_str(" world");
    name
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn read(x: bool) {
    if x {
        println!("x is true");
    }
}

