// Why does a future need to be 'static??
// - we are scheduling them to be run in the background
// - it outlives the scope of the function that invokes it
// - static tells borrow checker it will live forever

// can use runtime::block_on to run without 'static bound

// structured concurrency
// - every future created within a scope, that scope is 
// only finished once every future it contains is finished


// rust lifetimes and rop checker are all about automatic resource cleanup
// unstructured concurrency makes it impossible for compiler to clean up for us
// - rec counted pointers for cleanup at runtime

// structured concurrency
// Dynamic: do not know the number of futures at compile tile
// Staitc: know the exact number of futures at compile time

fn main() {
    println!("Hello, world!");
}
