use std::thread;
fn main() {
    // every program starts with one thread, the main thread
    // this executes the main function

    // spawn new thread
    // argument is function the new thread will exeucte
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("hello from the main thread");

    // wait for thread to finish executing and return std::thread::Result
    t1.join().unwrap();
    t2.join().unwrap();

    // can also pass a closure to a thread
    let numbers = vec![1, 2, 3];

    // execute thread with the closure and then join
    // spawn function has 'static lifetime bound on argument type
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();

    // get value out of thread by returning it from closure
    // will be in the Result in the join
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("Average: {average}");
}

fn f() {
    println!("Hello from another thread");

    // get the id of the thread
    let id = thread::current().id(); 
    println!("This is my thread id: {id:?}");
}
