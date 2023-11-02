fn main() {
    // release ordering prevents the initializaiton of the data from being reordered wit the store operation that shares the pointer with the other threads
    // acquire ordering prevents reordering that would cause the data to be accessed before the pointer is loaded

    // consume ordering is basically a lightweight variant of acquire ordering
    // synchronizing effects are limited to things that depend on the loaded value
    // achieved with the exact same instructions as relaxed ordering, can be "free"
    // although, no compiler actually implementes consume ordering...
    // compilers upgrade consume ordering to acquire ordering
    // may be implemented in the future
}
