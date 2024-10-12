
use futures::executor::block_on;
use futures::executor::ThreadPool;
use futures::future::join_all;
use futures::task::SpawnExt;

// async computation occurs within an executor
// - capable of spawning futures as tasks

async fn async_task(id: u32) -> u32{
    println!("Running in {id}");
    id
}

async fn async_task2() {
    println!("this does not return anything")
}

fn main() {
    // - most of the time, tasks should be executed on thread pool
    // - small set of worker threds can handle very large set of tasks
    let pool = ThreadPool::new().unwrap();
    // - spawn and run a task that does not return anything
    pool.spawn(async_task2()).unwrap();

    // - this task starts running async, get handle back even if result isnt finished
    // - can await the handle or use block on to get the result
    let handle = pool.spawn_with_handle(async_task(10)).unwrap();
    let handle_result = block_on(handle);
    println!("The result is {handle_result}");



    // - spawn tasks via spawn_obj method 
    // - if !Send, use spawn_local_obj

    // - can run task(s) on single thread with LocalPool
    // - allows you to spawn Non Send tasks
    // - good for i/o tasks w/ little work between I/O

    // - can just use block_on to run future to completion on current thread
    // - we are in sync context here to block_on lets us run future

    let single_future = async_task(10);
    let result = block_on(single_future);
    println!("The result is {result}");

    //  - if we were in async context, use join_all
    let futures = async {
        let tasks: Vec<_> = (1..=5).map(|i| {
            async_task(i)
        }).collect();
        let results = join_all(tasks).await;
        results.into_iter().sum::<u32>()
    };
    let result = block_on(futures);
    println!("The result is {result}");

}