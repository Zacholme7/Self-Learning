fn main() {
    // compiler may execute instructions out of order
    let mut a = 1;
    let mut b = 2;
    a += 1;
    b += 1;
    a += 1;
    // very contrived example, but could be a case where
    // the following happens due to compiler optimizations
    a += 2;
    b += 1;
    // further, the compiler may even execute the increment to b
    // before the increment to a
    // compiler can do whaterver it wants as long as the result does not change

    // when working with atomics, sometimes we want to have control
    // over the ordering

    // relaxed ordering (Ordering::Relaxed)
    // release and acquire ordering (Ordering::{Release, Acquire, AcqRel};
    // sequentially consistent ordering (Ordering::SeqCst}

    // rusts memory model is mostly copied from c++
    // abstract model with strict set of rules that attempt to represent..
    // .. the greatest common denomicator of all current and future archs..
    // .. while also givine the compiler enough freedom to make useful assumptions..
    // .. while analyzing and optimizing programs

    println!("Hello, world!");
}
