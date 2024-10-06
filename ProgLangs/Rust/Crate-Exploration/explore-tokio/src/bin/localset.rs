
use tokio::task::LocalSet;
use tokio::task;

// localset is a set of tasks that are executed on the same thread
// good for running a future that does not implement send
// can use to schedule one or more !Send futures on the same thread

use std::rc::Rc;

#[tokio::main]
async fn main() {

    let nonsend_data = Rc::new("this data is nonsend");

    // the following wont work becuase tokio expects a Send Future 
    // but RC makes it so it is !Send
    /* tokio::task::spawn(async move {
        println!("{}", nonsend_clone);
    }).await.unwrap() */

    // use local task send on thread calling runtime_block on (this is expanded from tokio::main)
    // use spawn_local to spawn !send futures
    let local = LocalSet::new();
    local.run_until(async move {
        let nonsend_data = nonsend_data.clone();
        // spawn local to ensure it is spawned on local task set
        task::spawn_local(async move {
            println!("{}", nonsend_data);
        }).await.unwrap();
    }).await;


}