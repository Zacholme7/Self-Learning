use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// this is our future
// arc that wraps a mutex of shared state so that state can be safely passed around threads
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

// shared state between the future and the waiting thread
struct SharedState {
    // if the sleep time has elapsed
    completed: bool,
    // waker for the task, used after completed is true
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = (); // the output of the future
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap(); // get the shared state

        // check for completion
        if shared_state.completed {
            // we are done, return ready
            Poll::Ready(())
        } else {
            // we are not ready, establish a waker
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
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













