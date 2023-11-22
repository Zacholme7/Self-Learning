// waiting for a thread to finish
// to avoid application being termined when an aception is throw
// need to call join in both cases so we dont skip it
// uses try/catch block to ensure that a thread with access to local state
// is finished before function exists
// verbose, see 2-3
#include <iostream>
#include <thread>

struct func {
    int& i; // local variable i

    // constructor
    func(int& i_)
       :i(i_) {}

    // operator overload
    void operator() () {
        for(unsigned j = 0; j < 1000000; j++) {
            do_something(i); // potential access to dangling reference
            i++;
        }
    }

    // just print the number
    void do_something(unsigned i) {
        std::cout << "number " << i << std::endl;
    }
};

void f() {
    int some_local_state = 0; // declare local state
    func my_func(some_local_state); // init a struct
    std::thread t(my_func); // start a thread
    try {
        std::cout << "in the main thread" << std::endl; // do something in main thread, pretend this can throw exception
    } catch(...) {
        t.join(); // join the thread in exception
        throw;
    }
    t.join(); // join the thread if we didnt not have an exception
}

int main() {
    f();
    return 0;
}
