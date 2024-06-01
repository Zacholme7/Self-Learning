use tokio::task::JoinSet;

// a collection of tasks spawned on a tokio runtime
// allow you to await completion of the tasks in the set
// tasks returned in the order that they are completed
// all tasks must have the same return type


#[tokio::main]
async fn main() {

        // create a new joinset
        let mut set = JoinSet::new();

        // spanw new tasks on the set
        for i in 0..10 {
                set.spawn(async move { i });
        }

        // await all of the results
        while let Some(res) = set.join_next().await {
                println!("{}", res.unwrap());
        }


}