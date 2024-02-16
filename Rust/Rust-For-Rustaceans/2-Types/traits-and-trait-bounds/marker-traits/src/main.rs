
// MARKET TRAITS
// ------------------
// - compile time guarantees about certain properties/behaviors of types
// - flags/markets for compiler
// - market traits are trais with no method or assocaited items
//  - used to indicate that something about that type implements them
//  - Send, Sync, Copy, Sized, Unpin, ...
// - also known as auto traits (except for copy) since compiler auto implements them

// consider api that manages ssh connections, can be authenticated or unauthenticated
struct Unauthenticated;
struct Authenticated;
// connection details
// -------------------

fn main() {
    println!("Hello, world!");
}
