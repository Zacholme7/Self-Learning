use tokio::sync;
// need synchronization primitives to let tasks communicate

#[tokio::main]
async fn main() {
    // move common form is message passing
    // message passing via channels

    // onshot channel
    // - sending single value from single producer to single consumer
    // - usually send res of computation to waiter

    let (tx, rx) = sync::oneshot::channel();

    // spawn task to send
    tokio::spawn(async move {
        if tx.send(3).is_err() {
            println!("the receiver dropped")
        }
    });

    // recv 
    match rx.await {
        Ok(v) => println!("got {}", v),
        Err(_) => println!("the sender dropped")
    }


}
