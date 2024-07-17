use std::thread;
use std::sync::Mutex;

// mutex -> mutual exclusion
// ensure threads have exclusive access to a piece of shared data
// can be locked or unlocked
// agreement between all threads that they will only access shared data when they have the mutex locked

fn main() {
    // 'std::sync::Mutex<T>', data can only be accesses via mutex
    // does not have unlock, lock() returns MutexGuard that is unlocked via dropping guard
    // behaves like an exclusive reference through DerefMut trait

    let n = Mutex::new(0);
    thread::scope(|s| {
        // spawn 10 threads
        for _ in 0..10 {
            s.spawn(|| {
                // lock the mutex and update state
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); // arent accessing shared data anymore so can drop guard
                // simulate some work
                thread::sleep(std::time::Duration::from_secs(1));
            });
        }
    });
    // we know all threads are done here, can remove protection via into_inner()
    // takes ownership of the mutex
    println!("{}", n.into_inner().unwrap());


    // a mutex gets marked as poisioned when a thread panics whild holding the lock
    // mutex will no longer be locked and calling lock method will result in ERR
    // protects against leaving data that is protected in an inconsistent state
    // calling lock() will still lock the mutex, the Err returned containst the mutex guard...
    // ...that allows us to correct the inconsistent state if needed

    let list = Mutex::new(Vec::<i32>::new());
    // can directly used without assigning name
    list.lock().unwrap().push(10);

    // temporaries will be dropped at end of statement, but leads to common pitfall
    if let Some(item) = list.lock().unwrap().pop() {
        // if we wanted to lock list, pop item, unlock list, and then do work with item,...
        // ...that is not what occurs here 

        // guard is not dropped until the end of the if statement
        // it is still locked here
    } // guard is dropped here


    // the above example does not happen here
    if list.lock().unwrap().pop() == Some(1) {
        // if condition is plain boolean, does not borrow anyything
        // no reason to extend lifetime of temporaries

        // if we used front() instead, the guard would have to stay around
        // lock is dropped before the body is executed
        // do something here
    }

    // correct way
    let item : Option<i32> = list.lock().unwrap().pop();
    if let Some(item) = item {
        // do some work
    }

}




































































