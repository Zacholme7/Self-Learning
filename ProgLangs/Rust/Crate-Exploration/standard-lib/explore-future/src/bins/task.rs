// the types and traits for working with asynchronous tasks


// futures are polled by executors to check if they are ready
// futures use wakers to notify executors when they might be ready

// context just provides access to a waker
// - waker used to wake up task when it is ready to make progress

// waker typically constructed by executor, wrapped in context,
// passed to Future::poll(), then if future returns pending it stores 
// waker and calls Waker::wake() when it should be polled again


use std::sync::Mutex;
use std::sync::Arc;
use std::task::{Context, Waker};
use std::pin::Pin;
use std::future::Future;
use std::task::Poll;
use std::thread;
use std::time::Duration;

struct SharedState {
    completed: bool,
    waker: Option<Waker>
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

#[tokio::main]
async fn main() {

}