use std::thread;
use std::sync::Mutex;
use std::process;
use std::time::Duration;
fn main() {
    // most commonly used too for sharing mutable data between threads is a mutex
    // job is to ensure threads have exclusive access to some data by blocking other threads that try to access it
    // two states: locked and unlocked
    // will block if trying to lock while it is already locked
    // simply an agreement that threads will only access the data when they have the mutex locked

    // mutex provided through std::sync::Mutex<T>
    // does not have an unlock() method to ensure that it can only be unlocked by the locker
    // instead, lock() returns MutexGuard which behaves like an exclusive reference
    // unlocking mutex is done by dropping the guard, Drop implementation will unlock it

    // everything pretty much happends serially, but just an example of mutex
    let n = Mutex::new(0); // mutex with the value 0
    // create a scoped lock 
    thread::scope(|s| {
        // spawn 10 threads
        for i in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap(); // thread will acquire lock and all other threads will bloc
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); // drop early to keep things going
                thread::sleep(Duration::from_secs(1)); // sleep this thread to show for example
            });
        }
    });
    println!("{}", n.into_inner().unwrap()); // into_inner takes ownerhsip of mutex 

    // lock poisoning
    // a mutex gets marked as poisoned when a thread panics while holding the lock
    // when that happends, mutex will no longer be locked, but calling lock will result in an error to indicate it has been poisoned
    // calling lock will still lock the mutex and the err returned contains the MutexGuard so we can correct inconsistent state

    // reader-writer lock
    // MutexGuard will provide us an exclusive reference (&mut T) even if we only wanted to look at the data
    // reader-writer lock is sligtly more complicated version of mutex
    // three states: unlocked, locked by a single writer, locked by any number of readers
    // useful for data that is read by multiple threads but only updated once in a while
    // provided through std::sync::RwLock<T>
    // has read() and write() method
    // two guards: RwLockReadGuard and RwLockWriteGuard
    // multithreaded version of RefCell
    // Both Mutex<T> and RwLock<T> require T to be Send
    // RwLock<T> also requires T to implement Sync
}
