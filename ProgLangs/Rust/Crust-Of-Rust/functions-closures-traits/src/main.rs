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
    
    // - function item coherces into function pointer and function pointer implements all three of
    // the above traits
    // - Fn and FnMut are subtraits of FnOnce, can be used where FnOnce is expected


    // this is a closure
    // takes arguments and returns a value
    // can capture things from env and generate unique function
    let f = || ();

    // this does capture from the environment
    // cannot represent a function pointer when it captures from env
    let z = String::new();
    let a = || {
        let _ = z; // this captures z from the env
    };

    /*
     * closure gets turned into something like this
    struct FClosue<'scope> {
        z: &'scope String
    }
    impl<'scope> Fn() for FClosure<'scope> {
        fn call(&self) {
            ...
        }
    }
    */

    let mut x = String::new();
    let b = || {
        x.clear();
    };

    // this is a constant closure that can be evaluated at compile time
    let x = || 0;

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

// can be used for dynamic dispatch
fn hello(f: Box<dyn Fn()>) {
    f()
}



// compile does not know if f can be called as const
/*
const fn fam<F: FnOnce()>(f: F) {
    f()
}
*/


// for any lifetime 'a f impl of F from str 'a to another &'a str
// way to say that this needs to hold for any lifetime
fn quo<F>(f: F)
where
    F: for<'a> fn(&'a str) -> &'a str,
{
}























