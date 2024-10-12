
// a stream of values produced asynchronously
// - async version of Iterator<Item = T>

// sequence of value producing events that occure async to the caller

use futures::Future;
use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep, Sleep};


struct Counter {
    count: u32,
    delay: Duration,
    delay_future: Pin<Box<Sleep>>
}

impl Counter {
    fn new(start: u32, delay: Duration) -> Self {
        Counter {
            count: start,
            delay,
            delay_future: Box::pin(sleep(delay)),
        }
    }
}

impl Stream for Counter {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.delay_future.as_mut().poll(cx) {
            Poll::Ready(_) => {
                let result = self.count;
                self.count += 1;
                self.delay_future = Box::pin(sleep(self.delay));
                Poll::Ready(Some(result))
            }
            Poll::Pending => Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let mut counter = Counter::new(0, Duration::from_secs(1));
    while let Some(number) = counter.next().await {
        println!("number {}", number);
    }




}