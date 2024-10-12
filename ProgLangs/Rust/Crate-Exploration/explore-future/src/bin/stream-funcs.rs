
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    // 
    let stream = stream::iter(1..=3);
    // convert into (next_item, tail_of_stream)
    // into_future moves the stream, so it needs to be !Unpin
    let (item, stream) = stream.into_future().await;

    






}
