
// async-std 
// - attempt to create alternative runtime that is closer to rust standard lib

// tokio is the canonical runtime, but it is more than just a runtime
// - so many more modules nowasays
// - makes a lot of assumptions on how async code should be written
// ex: says to enable full features. This sets up work stealing, multi threaded runtime
// which makes it so types are Send + 'static and have to use Arc/Mutex

// "Any time we reach for Arc or Mutex, stop and think about future implications
//of that decision"


fn main() {
    println!("Hello, world!");
}
