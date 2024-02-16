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
// -----------------------

fn main() {
    println!("Hello, world!");
}
