use anyhow::Result;
use task::*;
use tokio::task::spawn_blocking;
use tokio::task;
use std::rc::Rc;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};


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


    // await a localset
    let nonsend_data = Rc::new("this data is not send");
    let local = LocalSet::new(); // this is the local set to hold !Send
    // spawn a new task on the localset
    local.spawn_local(async move {
        println!("this is some nonsend data {}", nonsend_data);
    });
    local.await; // await the localset to run all futures to completion

    


    // demonstartion of LocalSet
    let local = task::LocalSet::new();
    let random_data = Rc::new("random data");
    // use run_until
    local.run_until(async move {
        let random_data_clone = random_data.clone();
        // spawn local makes sure it is spawned on local task set
        task::spawn_local(async move {
                println!("hello world {}", random_data_clone);
        }).await.unwrap();
    }).await;


    // what if we want to use !Send futures with LocalSet


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
