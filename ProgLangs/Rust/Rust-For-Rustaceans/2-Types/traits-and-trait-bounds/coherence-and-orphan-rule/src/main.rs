// COHERENCE and ORPHAN RULE
// --------------------------
// 1) can implement local trait for local type
// 2) can implement foreign trait for local type
// 3) cannot implement foreign trait for a foreign type unless one of the types involved is a local type

// 1)
trait MyTrait {
    // a local trait
    fn do_something(&self);
}
struct MyType; // a local type
impl MyTrait for MyType {
    fn do_something(&self) {
        println!("implementation of MyTrait");
    }
}

// 2)
impl std::fmt::Debug for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyType debug impl")
    }
}

// 3)
//impl std::fmt::Debug for Vec<i32> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(f, "Debug for Vec<i32>")
//    }
//}

// blanket implementation that allows you to implement trait over wide range of types
// implement for any type that implements std::fmt::Display
impl<T: std::fmt::Display> MyTrait for T {
    fn do_something(&self) {
        println!("implementation of MyTrait");
    }
}

//---------------------------------
fn main() {
    println!("Hello, world!");
}


/*
 *
- rust is pretty strict about where you can implement traits and what types you can implement them on
  - this is to preserver the coherence property: for any given type and method there is only one correct choice for which implemented of the method to use for that type
  - Ex: imagine if you could write your own Display trait for bool
- orphan rule established the balance: you can implmenet a trait for a type only if the trait or the type is local to your crate
  - allows you to implemented traits over a range of type with code like impl<T> MyTrait for T where T: ..., this is blanket implementation
 */
