use std::collections::VecDeque;
use std::thread;
use std::thread::park;
use std::sync::Mutex;
use std::sync::Condvar;
use std::time::Duration;

fn main() {
    // one way to wait for a notificaiton from another thread is thread parking
    // thread can park itself and go to sleep
    // another thread can then unpark the parked thread waking it up
    // parking is available through std::thread::park()
    // for unparking, you call unpark() on the thread object
    // thread object can be obtained from join handle or from std::thread::current()

    // example that uses mutex to share queue between two threads
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        // consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front(); // lock mutex, and get front of deque
            // if there is some item, consume it, otherwise park and wait for a new item
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }

        });
        for i in 0.. {
            queue.lock().unwrap().push_back(i); // lock mutex and push onto deque
            t.thread().unpark(); // unpark the waiting thread to run again
            thread::sleep(Duration::from_secs(1)); // sleep so other thread can regain control
        }
    });

    // important property of thread parking is that a call to unpark() before thread parks itself does not get lost
    // the request to unpark is still recorded
    // although, unpark requests do not stack up

    // condition variables
    // commonly used option for waiting for something to happen to data protected by a moutex
    // two operations: wait and notify
    // threads can wait on cv, after which they can be worken up when another thread notifies the same cv
    // multiple threads can wait on a cv
    // notificaiton can be sent to one waiting thread or all of them
    // can create cv for specific events or conditions
    // provided in std::sync::Condvar
    // wait method takes a MutexGuard, unlocks the mutex and goes to sleep
    // when woken up, it relocks the mutex and returns a new guard
    // two notifiy functions
    // nofity_one to wake up one waiting thread or notify_all to wake them all up

    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                dbg!(item);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })

}
