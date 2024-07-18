use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::sync::Condvar;
// when data is mutated by multiple threads, often need to wait for some event

fn main() {
    // thread parking: way to wait for a notificaiton from another thread
    // - can park iteself, put to sleep
    // - another thread can unpark a parked thread (wake it back up)
    // 'std::thread::park()'
    // - call 'unpark()' method on the thread you want to unpark
    //  - obtain via joinhandle or by thread itself via 'std::thread::current'
    println!("Hello, world!");

    // queue protected by a mutex
    let queue = Mutex::new(VecDeque::new());
    // this would still be correct without parking!!! but very inefficient
    // also breaks down in more complex situations, ex multiple producers and consumers
    thread::scope(|s| {
        // consume items from the queue
        let t = s.spawn(|| loop {
            // get an item from the queue
            let item = queue.lock().unwrap().pop_front(); 
            if let Some(item) = item { // if there is one, print it out
                println!("{item}");
            } else { // if there is not one, put this to sleep to wait for a new item
                thread::park();
            }
        });

        // producing
        for i in 0.. {
            // get the lock and push to the queue
            queue.lock().unwrap().push_back(i);
            // wake up the parked thread
            // a call to unpark before a thread parks itself does not get lost... cool
            // next time thread tries to park itself it will use the unpark right away
            // although, they do not stack
            t.thread().unpark();
            // sleep for a bit
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // condition variables are more commonly used for something waiting
    // have two basic operations, wait and notify
    // can wait on a cv and be notified when an event has occured
    // atomically unlock the mutex and start waiting, so notification cannot get lost
    // 'std::sync::Condvar'
    // 'wait()' takes a mutex guard, unlocked mutex and goes to sleep
    //  - when woken up, locks again and returns guard 
    // 'notify_one()' to wake up just one thread and 'notify_all()' for all
    // when condition is about data protected by a mutex, condvar is more convenient
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            // keep trying to get an item
            loop {
                // get the lock
                let mut q = queue.lock().unwrap();
                // while we have not got an item, sleep and then try to recieve
                let item: usize = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        // this is the sleeping, wait for a notification
                        q = not_empty.wait(q).unwrap();
                    }
                };
                drop(q);
                println!("{item}");
            }
        });

        // keep producing items
        for i in 0.. {
            // push an item one
            queue.lock().unwrap().push_back(i);
            // notify that we have a new item
            not_empty.notify_one();
            // sleep 
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
























