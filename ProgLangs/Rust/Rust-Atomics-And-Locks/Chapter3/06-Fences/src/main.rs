use std::sync::atomic::fence;
use std::sync::atomic::AtomicBool;

static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    // fence represents an atomic fence and is either a release fence (Release) an acquire fence (Acquire) or both (AcqRel or SeqCst)
    // allows you to separate the memory ordering from the atomic operation

    // release-store can be split into a release fence followed by a (relaxed) store
    // acquire-load can be split into a (relaxed) load followed by a acquire fence

    // single fence can be used for multiple variables

    // 10 threads do some work and store data in DATA
    // sets atomic boolean to indicate that the value is ready to be read by main thread
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i); // 
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed)); // checks which are ready
    
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }

}
