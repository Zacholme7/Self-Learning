use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};



struct SimpleFuture {
        state: i32
}

impl Future for SimpleFuture {
        type Output = i32;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                if self.state == 42 {
                        Poll::Ready(self.state)
                } else {
                        self.state += 1;
                        Poll::Pending
                }
        }
}


fn main() {
        
    println!("Hello, world!");

}
