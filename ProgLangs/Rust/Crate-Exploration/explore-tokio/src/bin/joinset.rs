use tokio::task::JoinSet;

// a collection of tasks spawned on a tokio runtime
// allow you to await completion of the tasks in the set
// tasks returned in the order that they are completed
// all tasks must have the same return type


async fn task(i: u32) -> u32 { i * 2 }

#[tokio::main]
async fn main() {

    // create a new joinset and put 10 tasks on it
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(task(i));
    }


    // get the results as they are done
    while let Some(res) = set.join_next().await {
        println!("The result is {:?}", res);
    }
    // in comparison, join_all would return when all are done



}