use std::future;


// Future represents async computation obtained by use of async
// - might not have finished computing yet

// the core of a future is the poll method
// - tries to resolve future into a final value
// - wont call poll directly, but use .await
// - returns either
//  - Poll::Pending: the future is not ready yet
//  - Poll::Ready(val): the future is finished and we have the result

// if future is not ready, it clones Waker from the Context
// when signal arives and it is ready, Waker::wake is called and task is awoken
// then poll to get the value

// poll is not called repeatedly in loop, only called when progress can be made

// want to express a value that is not quite ready

// async keyword: turn code into future that must be awaited to run
// .await: suspend function execution until executor has run it to completion

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::poll_fn;

struct MyFuture;


impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42)
    }
}

#[tokio::main]
async fn main() {
    // create a future that will never resolve
    //let future = future::pending();
    //let () = future.await; // this will never resolve

    // PollFn allows you to create future from a closure
    // Wraps a function returning poll
    // creates a future from a closure that implements polling behavior
    let future = poll_fn(|ctx| Poll::Ready(42));

}