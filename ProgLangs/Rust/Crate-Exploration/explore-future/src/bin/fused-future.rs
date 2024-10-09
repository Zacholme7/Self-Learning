// a future that tracks whether or not the underlying future should no longer be polled
// - is_terminated() will return true if it should not be polled
// - prevent issue when polling future after it is already ready

use futures::future::{self, Fuse, FusedFuture};
use futures::FutureExt;
use std::pin::pin;
use futures::executor::block_on;

async fn async_task() -> i32 {

    println!("Executing the task");
    10
}

fn main() {

    let mut fused_future = async_task().fuse();

    // this is the first poll and will drive it to completion
    if !fused_future.is_terminated() {
        let result = block_on(pin!(&mut fused_future));
        println!("The result is {}", result);
    }

    // this is the second poll but it is already terminated
    if fused_future.is_terminated() {
        println!("the future is terminated, wont execute again");
    }

}