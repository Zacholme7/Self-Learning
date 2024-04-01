// notes
// -------------------------
// - ? operator acts as shorthand for unwrap or return early
//  - perfoms type conversion through the from trait
//  - func that returns Result<T, E>, can use on any Result<T, X> where E: From<X>
// - implement From trait and use Into in bounds
// - ? is just syntax sugar for trait called Try

fn main() {
    println!("Hello, world!");
}
