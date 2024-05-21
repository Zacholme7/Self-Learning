use anyhow::Result;
use task::*;
use tokio::task::spawn_blocking;
use tokio::task;


#[tokio::main]
async fn main() -> Result<()> {
    // basic example of scheduling a task
    let handle = spawn(async {
        "hello world"
    });
    let result = handle.await;
    

    // computer factorial is blocking, so we use spawn blocking
    // this is good for long running cpu intensive blocking operations. 
    // A new thread pool is created
    let n = 20;
    spawn_blocking(move || compute_factorial(n))
        .await
        .unwrap();



    // on the other hand, block in place is for when you want to convert the 
    // current thread to a blocking thread and moving all tasks to another
    // worker thread. use when you are in async context and need temp blocking operation
    let result = task::block_in_place(|| {
        compute_factorial(20)
    });


    // by wrapping in task::unconstrained, allowed to run to completion without yielding
    // can cause starvation if let run too long
    let task = task::unconstrained(long_running_task());
    task.await;

    Ok(())
}

// this is an example of a blocking (cpu intensive) function
// therefore, it is a good candidate for spawn_blocking
fn compute_factorial(n: u64) -> u64 {
    (1..=n).product()
}


async fn long_running_task() {
    let mut count = 0;
    for _ in 0..1_000_000_000 {
        count += 1
    }
}
