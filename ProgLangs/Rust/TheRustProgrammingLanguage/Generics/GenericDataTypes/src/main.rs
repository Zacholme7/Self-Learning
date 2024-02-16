// generic struct definition
struct Point<T, U> {
    x: T,
    y: U,
}

// generic enum defintion
enum Option<T> {
    Some(T),
    None,
}

// can use generics in method defintions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// can also specify constraits for generic types for methods
// for example, can only call this method on point with f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // calling generic function
    let number_list = vec![1, 2, 3];
    let result = largest(&number_list);

    // instantiating generic struct
    let integer = Point {x: 5, y:10};
    let float = Point {x: 5.1, y: 2.1};

    // no performance cost for using generics
    // performs monomorphization at comple time
}

// for function that uses generics
// place generic in signature of function where we would put data types
// place type name declations in angle brackets
// largest is a generic over some type t, list is slice of values of type T
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largets = item;
        }
    }
    largest
}
