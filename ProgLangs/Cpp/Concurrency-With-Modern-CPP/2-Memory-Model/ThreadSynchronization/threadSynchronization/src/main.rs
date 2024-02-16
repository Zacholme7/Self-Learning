use std::sync::mpsc;
use std::thread;

fn prepare_work(sender: mpsc::Sender<Vec<i32>>) {
    let data = vec![1, 2, 3];
    println!("Sender: data prepared");
    sender.send(data).unwrap();
}

fn complete_work(receiver: mpsc::Receiver<Vec<i32>>) {
    println!("Waiter: waiting for data");
    let mut my_vec = receiver.recv().unwrap();
    my_vec[2] = 2;
    println!("Waiter: completed the work.");
    for i in my_vec {
        print!("{} ", i);
    }
    println!();
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let t1 = thread::spawn(move || {
        prepare_work(tx);
    });

    let t2 = thread::spawn(move || {
        complete_work(rx);
    });

    let _ = t1.join();
    let _ = t2.join();
}
