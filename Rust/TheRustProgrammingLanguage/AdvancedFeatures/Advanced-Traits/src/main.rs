fn main() {
    // associated types connect a type placeholder with a trait such that the trait method defs can use the placeholder type in the signature
}

pub trait Iterator {
    type Item; // associated type

    // implementor of trait will specify concrete type for item
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // implementation
    }
}

// why not just use generics??
// must annotate the types in each implementation
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}


// when using generic type parameters, can spcify default concrete type for generic type
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // associated type named output
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

trait Add<Rhs=Self> { // default type parameter
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}