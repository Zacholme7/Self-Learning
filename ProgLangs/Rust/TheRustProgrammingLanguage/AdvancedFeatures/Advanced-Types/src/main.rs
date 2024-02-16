use std::fmt;
use std::io::Error;

fn main() {
    // can declare type alias to give existing type another name
    type Kilometers = i32; // kilometere now a synonym for i32, not a seperate new type

    let x: i32 = 5;
    let y: Kilometers = 5;
    // dont get typechecking benefits here though, can interchange i32 and kilometers


    // dynamically sized types and the sized trait
    // some value sizes can only be known at runtime

    // does not work, need to known the sizes
    let s1: str = "hello there";
    let s2: str = "hello there again";

    // have to make references so we just store the starting position
    let s1: &str = "hello there";
    let s2: &str = "hello there again";

    // rust provides sized trait to determine wherther or not a types size is known at compile time
    // auto implemented for everything whose size is known at compile time
    // also added to bound generic functions
    // 2
}
// 2
// this function
fn generic<T>(t: T) {
    //...
}
// is actually treated as
fn generic<T: Sized>(t: T) {
    // ...
}
// by default, generic will only work on types that have known size at compile time
// can relax this restruction
// means that T may or may not be sized
fn generic<T: ?Sized>(t: &T) {
    // ...
}


// type aliasas commonly used with result type
type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// rust have special type named ! that is known as empty time
// we call it never type because it stands in the place of the return type when function will never return
// ! can be coercied into any time
fn bar() -> ! { // the function bar never retrusn, called diverging function
    // ...
}