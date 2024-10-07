
use tokio::sync;
use tokio::time::{interval, sleep, Duration};


struct SendOnDrop {
    sender: Option<sync::oneshot::Sender<&'static str>>,
}

impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send("I got dropped");
        }
    }
}

#[tokio::main]
async fn main() {
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


    // can use in tokio select, just have to use &mut in channel
    let (send, mut recv) = sync::oneshot::channel();
    let mut interval = interval(Duration::from_millis(100));

    tokio::spawn(async move { // send a message
        sleep(Duration::from_secs(1)).await;
        send.send("shut down").unwrap();
    });

    loop {
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut recv => {
                println!("Got a new message {}", msg.unwrap());
                break;
            }
        }
    }

    // can send when being dropped
    let (send, recv) = sync::oneshot::channel();
    let send_on_drop = SendOnDrop {sender: Some(send)};
    drop(send_on_drop); // this will invoke drop and send the message
    println!("{:?}", recv.await);







}

