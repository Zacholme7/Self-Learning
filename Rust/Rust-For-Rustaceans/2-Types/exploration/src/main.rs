#[repr(C)]
struct Foo {
    tiny: bool,  // sees tiny field whose size is 1 bit, but given 1 byte
    normal: u32, // 4 bytes but tiny is 1 which gives us misalignment, compiler pads with 3 bytes
    // to make tiny + normal 8 bytes
    small: u8,  // 1 byte and aligned so it is okay
    long: u64,  // not 8 byte aligned, so have to add 7 bytes to make long be aligned
    short: u16, // just need 2 byte alignmetn here
} // align foo, which will follow the biggest alignment (8)
  // total is 32 bytes bytes
  // 1 (tiny) + 3 (padding) + 4 (u32) + 1(small) + 7 (padding) + 8 (long) + 2 (short) + 6 (padding)

// example of associated type
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

// example of a generic type
trait Printer<T> {
    fn print(item: T);
}
struct Screen;
impl Printer<String> for Screen {
    fn print(item: String) {
        println!("printing to screen: {}", item);
    }
}

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

// TRAIT BOUNDS
// -------------------
// dont need to involve generic type param dirctly
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

// instead of specifying each bound individually...
// fn example<T: Hash + Eq, S: BuildHasher + Default>() {...}
// you can specify a bound that relates to your use case
fn example<T, S>()
where
    HashMap<T, usize, S>: FromIterator<(T, usize)>,
{
}

// bounds on associated types of types your generics work over is allowed
trait Iterable {
    type Item;
    fn iter(&self) -> Box<dyn Iterator<Item = Self::Item>>;
}
fn flatten<I>(iter: I)
where
    I: Iterable,
    I::Item: IntoIterator,
{
}

// Higher ranked trait bounds
// allow specifying bounds that must hold for all lifetimes
// useful for references and closures
fn call_with_ref<F, T>(f: F, arg: &T)
where
    F: for<'a> Fn(&'a T),
{
    f(arg)
}

// bringing it all together
// generic function that can debug print any iterable collection
// shows generic implementation of Debug for any type T that can be turned into an iterator
// where the items of the iterator implement debug
use std::fmt::{self, Debug, Formatter};
struct AnyIterable<T>(T);
impl<T> Debug for AnyIterable<T>
where
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

fn main() {
    println!("Hello, world!");
}
