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

// want to express a value that is not quite ready


fn main() {

}