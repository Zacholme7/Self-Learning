use crate::lib::LimitTracker;
mod lib;
fn main() {
    // interior mutability is a design pattern that allows you to mutate data even when there are immutable references to that data
    // the pattern uses unsafe code inside a data structure to bend rusts rules that goven mutation and borrowing
    // have to ensure that the borrowing rules will be followed at runtime
    // RefCell<T> type follows the interior mutability pattern

    // RefCell<T> represents single ownership over the data it holds
    // recall borrowing rules
    // 1) you can have either one mutable reference or any number of immutable references
    // 2) references must always be valid
    // With RefCell<T>, the borrowing rule invariants are enforced at runtime
    // if you break the rules the program will panic rather than not compile
    // advantage of checking at runtime is that certain memory safe operations are allowed which would have been rejected by compiler
    // RefCell<T> is useful when youre sure your code follows the borrowing rules but the compiler is unable to guarantee that
    // Only for use in single threaded scenarios

    // Recap of what to choose
    // enables multiple owners of the same data, Box<T> and RefCell<T> only have single owner
    // allows immutable or mutable borrowrs checked at compiler time, Rc<T> only allowes immutable borrows checked at compile time and RefCell<T> allows immutable or mutable borrows checked at runtime
    // can mutate value inside of RefCell<T> when when it is immutable 

    // iterior mutability


}
