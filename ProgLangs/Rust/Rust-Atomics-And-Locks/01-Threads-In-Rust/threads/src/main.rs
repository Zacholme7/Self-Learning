// every program starts with one thread, which is the main thread

// threading crate
use std::thread;
fn main() {
    // spawn a new thread with 'thread::spawn'
    thread::spawn(f);
    thread::spawn(f);

    // * sidenote, println calls std::io::Stdout::lock() to make sure it does not get interrupted
    println!("I am the main thread");

    // we need to join threads to ensure they are completed before the main thread returns
    // t1 is a 'JoinHandle', we can call 'join()' on a joinhandle. this will make it for the
    // main thread to wait for the newly spawned thread to return before it continues
    let t1 = thread::spawn(f);
    t1.join().unwrap();

    // can also pass closures to threads, allows the captures of variables
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    // cannot call this print function since numbers is moved into the closure
    // if we leave off the 'move' keyword, this would still not compile. The thread
    // would take a reference to numbesr, but what if we droped numbers while the
    // thread still had access to it??
    // println!("{:?}", numbers);

    // a thread may run till the end a programs execution
    // due to this, a thread is required to have a 'static lifetime bound on its arguments.
    // A reference may not live forever, this is a no no

    // threads can return values and you can access that value via joining
    let numbers = Vec::from_iter(0..=100);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();

    // 'thread::spawn' is shorthand for 'std::thread:Builder::new().spawn().unwrap()'
}

fn f() {
    // this will get the id of the thread
    // ThreadId can be copied around and used for equality checks
    // not guaranteed to be assigned consecutively
    let id = thread::current().id();
    println!("I am thread {id:?}");
}
