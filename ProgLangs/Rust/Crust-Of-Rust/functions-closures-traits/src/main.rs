fn main() {
    println!("Hello, world!");
    // what is the type of x?
    // a function item, not a function pointer
    // - a zero sized value that references the unique function bar
    let x = bar;

    // have to define the generic on it
    let y = foo::<i32>;

    // compiler will coherce function item type into function pointer type
    baz(moo::<i32>);
    baz(moo::<u32>);

    // function item uniquely identifies a particular instance of a function whereas a function
    // pointer is a pointer to a function with a given signature. Can item into pointer but not
    // pointer into item


    // saying that quox is a function that is generic over some f where F implements Fn trait.
    // Fn() != fn()
    // - fn(): function pointer
    // - Fn(): trait bound
    quox(foo::<u32>);

    // the different function traits
    // Fn(): takes a shared reference to self, can be called multiple times at same time
    // FnMut(): takes a exclusive reference to self, can only be called once at a time
    // FnOnce(): takes owned reference to self, can only be called once since it moves value


}

fn bar() {}

fn foo<T>() {}

// takes function pointer
fn moo<T>(_: u32) -> u32 {
    0
}
fn baz(f: fn(u32) -> u32) {}

fn quox<F>(f: F)
where
    F: Fn(),
{
}
