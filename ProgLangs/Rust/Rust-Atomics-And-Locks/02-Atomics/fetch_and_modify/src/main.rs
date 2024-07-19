use std::sync::atomic::AtomicUsize;
use std::thread;
use std::sync::atomic::Ordering;
fn main() {
    // modify the atomic variable but also load (fetch) the original value
    let a = AtomicUsize::new(100);
    let b = a.fetch_add(10, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);
    assert_eq!(b, 100);
    assert_eq!(c, 110);

    // example of progress tracking
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });


}
