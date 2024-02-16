// a future does nothing until it is awaited
// future just describes a series of steps that will be executed
//
use std::future::Future;


fn main() {
    println!("Hello, world!");

    let x = await foo1(); // this is not a usize
}

// these two are equivalent
async fn foo1() -> usize {
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {
        0
    }
}

