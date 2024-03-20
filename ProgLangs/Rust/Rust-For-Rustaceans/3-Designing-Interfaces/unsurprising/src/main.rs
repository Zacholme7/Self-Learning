// Notes
// -----
// - interface should be intuitive enough that if a user has to guess, they usually guess correctly
// - anything that can be unsurprising should be
// - if you reuse names, user should be able to make basic assumptions from other names
//  - method called iter probably takes &self and gives you an iterator
//  - things that share a name should also work the same way
// - should implement some standard traits
//  - Debug, Send, Sync, Clone, Default, Comparison traits (PartialEq), Serialize, Deserialize
// - what about copy?
//  - users do not generally expect copy, generally expect that if they want a copy they should
//  call clone
//  - very easy for a type to stop being copy (Ex: the type changes to need to hold a String)
// - when you define new trait, you usually want to provide blanket impl for &T where T: Trait,
// &mut T where T: Trait and Box<T> where T: Trait
// - Deref and AsRef provide something like inheritance
//  - have val of type T and call methods on some type U
//  - called direclty on T-typed value if T: Deref<Target = U>

fn main() {
    println!("Hello, world!");
}
