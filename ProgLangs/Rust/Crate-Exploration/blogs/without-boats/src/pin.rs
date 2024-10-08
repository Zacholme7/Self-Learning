// future type has state for each possible step it can pause
// - start, every await, when it finished
// when you write async function, compiler transforms it into 
// state machine where the state is often self referential

/*
async fn foo<'a>(z: &'a mut i32) { ... }

async fn bar(x: i32, y: i32) -> i32 {
    let mut z = x + y;
    foo(&mut z).await;
    z
}

// could transform into this
enum Bar {
    // When it starts, it contains only its arguments
    Start { x: i32, y: i32 },
    // At the first await, it must contain `z` and the `Foo` future
    // that references `z`
    FirstAwait { z: i32, foo: Foo<'?> }
    // When its finished it needs no data
    Complete,
}
*/

// if bar in first await state, then moves, then we have dangling pointers
// need to restrict it from moving, or pin it



// there may be cases where needs self referential data which is in 
// the internal state. this is why future types are self referential

// goal of Pin is to make it safe to manipulate self referential types 
// generated by compiler from async func or impl with unsafe code

// Pin: new category of typestate which puts obj into pinned typestate 
// when the ref is created
// - represented with the pin type
// - implemented as library, so if you need to mutate can do it via unsafe

// unpin auto trait allow getting mutable reference from a pinned pointer without
// usafe code if the type cant be self referential


// reborrowing: when mutable ref is used, it will functionally insert
// a reborrow to borrw the reference again instead of moving it

use std::pin::Pin;
use std::future::Future;

async fn example() -> i32 { 32 }

fn main() {
    // how to create a pin
    let x = 5;
    let pin = Pin::new(&x);
    let pinned_future = Pin::new(Box::new(example));

    // 2 guarantees
    // 1) A pinned reference cannot be moved


}


// what are the two guarantees pin makes
// 1) A pinned reference can never be moved
// 2) If t: !Unpin, then &mut T only provides shared access to the pined value