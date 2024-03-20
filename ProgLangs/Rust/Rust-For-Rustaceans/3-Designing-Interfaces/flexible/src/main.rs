// Notes
// -------
// - avoid imposing unneccessary restrictions and only make promises you can keep
// - restrictions usually come in the form of trait bounds and arg types
// - promises come in form of trait impls and return types
// - in most cases it pays off to use generics rather than concrete types
//  - make arg generic if you can think of other types a user might reasonably and frequently want
//  to use instead of concrete type
// - should prefer traits to be object safe even if that comes at slight cost to ergonomics of
// using them
// - if code needs ownership of data, it must store the owned data
//  - caller should provide owned data
// - if code does not need owned data, just take reference
// - provide best effort cleanup via drop
// - can also provide explicit destructor, method taht takes ownership of self and exposes errors
// via -> Result<_, _>

fn main() {
    println!("Hello, world!");
}
