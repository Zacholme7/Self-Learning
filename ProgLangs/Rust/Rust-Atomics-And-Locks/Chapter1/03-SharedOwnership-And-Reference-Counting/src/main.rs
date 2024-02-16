use std::thread;

fn main() {
    // several ways to create something thats not owned by a single thread
    // static value is owned by the entire program

    // both threads can access x but neither own it
    static X: [i32; 3] = [1, 2, 3];
    thread::spawn(|| dgb!(&X));
    thread::spawn(|| dgb!(&X));

    // can share ownership by leaking an allocation
    // use box::lead to release ownership of a box
    // the box will live forever wtihout a owner
    // leaking memory with this approach
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dgb!(&x));
    thread::spawn(move || dgb!(&x));

}
