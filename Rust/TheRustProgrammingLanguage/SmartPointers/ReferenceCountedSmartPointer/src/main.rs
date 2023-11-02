enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // cases when a single value might have multiple owners
    // enable multiple owners with Rc<T>
    // keeps track of the number of refernces to a value to determine whether or not the value is still in use
    // if there are zero references, then the value is cleaned up
    // only for use in single threaded scearios


    // creat two lists that share ownership of a third list
    // whenerver we call Rc::clone, reference count to the data is increased
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
