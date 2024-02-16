// goes through each variant and sees how much space it needs
// Box gives a size that can be determined for recursive variants
// box provides an idirection and heap allocation
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // pointer is a general concept for a variable that contains an address in memory
    // address points to other data
    // most common type of pointer in rust is a reference

    // smart pointers act like a ointer but have additional metadata and capabilities
    // in many cases, smart pointers own the data
    // implement the deref and drop trait

    // most straightforward pointer is a box, written "Box<T>"
    // allows you to store data on the deap rather than the stack
    // on the stack is a pointer to the heap data
    // dont have performance overhead, but dont have many extra capabilities
    // use cases
        // have type whose size cant be known at compile time
        // large amount of data that you want to transfer ownership and ensure it wont be copied
        // when you want to own value and care its type implements a trait rather than being a specific type

    // using a box<T>
    let b = Box::new(5); // 5 is allocated on the heap and b is a pointer to that value
    println!("{}", b);
    
    // boxes enable recursive types, another value of the same type as part of itself
    // ex: cons type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));



}
