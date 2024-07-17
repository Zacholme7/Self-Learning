use std::thread;
use std::rc::Rc;

// two traits to keep track of what types can be safely used across threads
// - send: type is send if it can be sent to another thread, ownership is transferred
// - sync: type is sync if it can be shared with another thread, T is sync iff &T is Send

// all primitives types are sync and send
// Sync and send are auto traits, so they are usually auto implemented
// can opt out by adding field that does not implement the trait
// - std::marker::PhantomData<T> comes in hande for this, treated by compiler at T but does not exist at runtime
//  - is zero sized, so it takes no space
//  - used to mark things that are not actually there, but want compiler to think they are
//  - sometimes need to tell compiler about types/behaviors that are important for type checking but dont exist

// raw pointers (*const T and *mut T) are not Send or Sync

// can opt into the traits
struct X{
    p: *mut i32,
}

// notice how they require the unsafe keyword
unsafe impl Send for X {}
unsafe impl Sync for X {}


fn main() {
    println!("Hello, world!");
    let a = Rc::new(10);
    // thread::spawn requires the argument to be Send
    // a closure is only send if all captures are Send
    //thread::spawn(move || { // this is an error, Rc is not Send
    //    println!("{a}");
    //});
}
