use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // when sharing data between threads and neither of them is guaranteed
    // to outlive the other, neither cna own the data. need to live as long as the 
    // longest living thread

    // statics are the easiest way to guarantee that something will live for the entire program
    // not owned by a single thread

    // "owned" by the entire program,
    // both threads can access X, but neither of them own it
    // constant initializer, never dropped, exists before the main function
    static X: [i32; 3] = [1, 2, 3];
    thread::spawn(|| println!("{:?}", &X));
    thread::spawn(|| println!("{:?}", &X));
    println!("{:?}", X);

    // can share ownership via leaking
    // Box::leak can leak ownership of a box, promising to never drop it
    // Box will then live forever without an owner
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // move makes it look like we are moving it in, but x revels that we are only giving it reference
    thread::spawn(move || println!("{:?}", x));
    thread::spawn(move || println!("{:?}", x));
    // you are leaking memory with box..
    // static does not mean it lived since the start of the program, but only that it lives till the end
    
    // ** leaking allows for runtime init **
    // static declares a static varaible
    // &'static is a reference with a static lifetime

    let global_string: &'static str = Box::leak(String::from("blah, World!").into_boxed_str());
    let print_global = move || {
        println!("{}", global_string);
    };
    thread::spawn(move || print_global()).join().unwrap();

    // we can share ownership/keep track of owners to make sure value is dropped when no owners are left
    // 'std::rc::Rc' reference counted, will not allocate anything new but increment a counter
    // original and cloned RC refer to the same value
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    // however, Rc is not thread safe, may try to modify reference counter at the same time

    // can use 'std::sync::Arc', atomically reference counted
    // the counter can be modifed, guarantees that modifications to ref counter are atomic 
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    thread::spawn(move || println!("{:?}", a));
    thread::spawn(move || println!("{:?}", b));

    // can get annoying naming clones, just wrap in scope
    let a = Arc::new([1, 2, 3]);
    thread::spawn({
        let a = a.clone();
        move || {
            println!("{:?}", a);
        }
    }); 
    println!("{:?}", a);
}
