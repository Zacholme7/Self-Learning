use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn main() {
    // checks if the atomic value is equal to a given value, only if that is the case does...
    // ... it replace it with a new value. all atomically. returns the previous value and sucess/fail
    // can load value atomic variable, do calculation, and only store it if the value did not change

    // compare_exchange_weak will sometimes leave the value untouched and return ERR, event if the...
    // ..value matched the expected calue
}

fn increment(a: &AtomicUsize) {
    // load the current value
    let mut current = a.load(Ordering::Relaxed);
    loop {
        // do our calculation
        let new = current + 1;
        // compare then exchange
        // if Ok, then we were able to exchange
        // if Err, then we were not able to exchange and current value returned
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}


fn get_key() -> u64 {
    static KEY: AtomicUsize = AtomicUsize::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = generate_random_key(); 
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) { 2
            Ok(_) => new_key, 
            Err(k) => k, 
        }
    } else {
        key
    }
}