
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // - a multiproducer multiconsumer broadcast queue
    // - sender broadcasts to all receiver values, sender can be cloned
    // - sender and receiver is Send + Sync as long as T is Send 
    // - value stored in channel and cloned on demand for each receiver
    // - messages are retained until all receivers recieve a cloned
    //  - if one is stalled, this can cause lagging
    
    let (tx, mut rx) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx.recv().await.unwrap(), 10);
        assert_eq!(rx.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });


    tx.send(10).unwrap();
    tx.send(20).unwrap();





}
