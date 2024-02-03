use std::sync::{Arc, Mutex, Condvar, mpsc};
use std::thread;


fn main() {
    // the cpp based way
    let data = Arc::new((Mutex::new(vec![0, 0, 0]), Condvar::new()));
    let data_ready = Arc::new(Mutex::new(false));

    // clone the arcs so they can be moved into the thread
    let data_t1 = data.clone();
    let data_ready_t1 = data_ready.clone();
    let t1 = thread::spawn(move || {
        let (lock, cvar) = &*data_t1; // get the mutex with the vec and the condvar
        let mut data_ready = data_ready_t1.lock().unwrap(); // get the ready boolean
        // wait to be notified
        while !*data_ready {
            data_ready = cvar.wait(data_ready).unwrap();
        }
        let mut data = lock.lock().unwrap();
        data[1] = 2;
        println!("Work done");
    });

    // clone the arcs so they can be moved into the thread
    let data_t2 = data.clone();
    let data_ready_t2 = data_ready.clone();
    let t2 = thread::spawn(move || {
        let (lock, cvar) = &*data_t2;
        {
            let mut data = lock.lock().unwrap();
            *data = vec![1, 0, 3];
            println!("Data prepared");
        }
        {
            let mut data_ready = data_ready_t2.lock().unwrap();
            *data_ready = true;
        }
        cvar.notify_one();
    });

    let _ = t1.join();
    let _ = t2.join();

    let (lock, _) = &*data;
    let data = lock.lock().unwrap();
    println!("{:?}", *data);


    // the more idiomatic rust way to do it
    let (tx, rx) = mpsc::channel();

    let t3 = thread::spawn(move || {
        let mut data: Vec<i32> = rx.recv().unwrap();
        data[1] = 2;
        println!("Work done: {:?}", data);
    });

    let t4 = thread::spawn(move || {
        let data = vec![1, 0, 3];
        println!("Data Prepared");
        tx.send(data).unwrap();
    });

    let _ = t3.join();
    let _ = t4.join();
}
