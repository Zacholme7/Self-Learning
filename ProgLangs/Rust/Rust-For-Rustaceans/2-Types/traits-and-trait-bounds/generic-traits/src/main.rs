// Notes: Generic Traits
// ----------------------
// - rust traits can be generic in two ways
// 1) with generic type parameter. Ex: trait Foo<T>
//   - have to specify all generic parameters and repeat any bounds on the parameters
// 2) associated types. Ex: trait Foo { type Bar; }
// - use an associated type if you expect only one implementation of the trait for a given type and use generic type parameter otherwise

// this is an example of a generic trait with an associated type
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
}
struct IntegerContainer(Vec<i32>);
impl Container for IntegerContainer {
    type Item = i32;
    fn add(&mut self, item: Self::Item) {
        self.0.push(item);
    }
}

// this is an example of a generic trait with a generic type parameter
// want to use it for multiple types, so we go with generic type parameter
trait Printer<T> {
    fn print(item: T);
}
struct Screen;
impl Printer<String> for Screen {
    fn print(item: String) {
        println!("printing to screen: {}", item);
    }
}
impl Printer<bool> for Screen {
    fn print(item: bool) {
        println!("printing to screen: {}", item);
    }
}


fn main() {
    println!("Hello, world!");
}
