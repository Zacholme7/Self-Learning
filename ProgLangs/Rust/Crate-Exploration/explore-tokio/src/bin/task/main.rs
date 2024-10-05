

// task: lighweight non blocking unit of execution, similar
// to an OS thread but managed by a runtime
// - light weight. Since they are scheduled and created by runtime
// they do not require a context switch
// - cooperativley scheduled: allowed to run until it yields. 
// - nonblocking: if cannot keep executing, yields and other task 
// gets scheduled

use tokio::task;

#[tokio::main]
async fn main() {
    // async equivalent to thread::spawn
    // takes async block or other future
    task::spawn(async {
        // you perform some work here
    });

    // spawn returns a joinhandle that can be awaited
    let join = task::spawn(async {
        "hello world"
    });
    let res = join.await.unwrap();
    assert_eq!(res, "hello world");

    // if we need to use something blocking, use 
    // spawn_blocking or block_in_place

    // spawn a blocking function on a dedicated thread pool for blocking tasks
    let join = task::spawn_blocking(|| {
        // this is some blocking/sync code
        "blocking complete"
    });
    let result = join.await.unwrap();

    // block in place is available on the multithreaded runtime
    // works by transitioning current worker thread to blocking thread, moving
    // other tasks on that thread to another worker thread
    let result = task::block_in_place(|| {
        // do some compute-heavy work or call synchronous code
        "blocking completed"
    });
    
    assert_eq!(result, "blocking completed");



}