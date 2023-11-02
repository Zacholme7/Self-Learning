use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;
fn main() {
    // modify the atomic variable but also load (fetch) the original value as a single atomic operation
    /* 
    pub fn fetch_add(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_sub(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_or(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_and(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_nand(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_xor(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_max(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_min(&self, v: i32, ordering: Ordering) -> i32;
    pub fn swap(&self, v: i32, ordering: Ordering) -> i32; // "fetch_store"
    */

    // progress reporting from multiple threads
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // spawn 4 threads
        for t in 0..4 {
            // move the t into it
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed); // add 1 to the num_done atomic with a fetch_add
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed); // load the atomic and check progress
            if n == 100 { break; }
            println!("Working... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    })


    // statistics example
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // spawn 4 thread to do work
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t + 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed); // increment the number done
                    total_time.fetch_add(time_taken, Relaxed); // add the amount of time taken
                    max_time.fetch_max(time_taken, Relaxed); // set the max time
                }
            });
        }

        loop {
            let total_time = Duration:from_micros(total_time.load(Relaxed)); // load the total time
            let max_time = Duration::from_micros(max_time.load(Relaxed)); // load the max time
            let n = num_done.load(Relaxed); // load the number done
            if n == 100 {break;} // if all items are done brea
            if n == 0 {
                println!("Working... none done yet");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");




}
