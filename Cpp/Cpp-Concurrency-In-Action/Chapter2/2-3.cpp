// 2.3: using RAII to wait for a thread to complete
// ---------------------
// can use Resource Acquisiting is Initialization (RAII)
// provide a class that does the join() in its desructor
#include <iostream>
#include <thread>

class thread_guard {
    std::thread& t; // reference to thread
public:
    // explicit constructor
    explicit thread_guard(std::thread &t_):
        t(t_) {}

    // destructor
    ~thread_guard() {
        // upon destruction of the class, we want to join the thread
        if(t.joinable()) {
            t.join();
        }
    }

    // delete copy and copy assignment constructors
    thread_guard(thread_guard const&) = delete;
    thread_guard& operator=(thread_guard const&) = delete;
};

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
    int some_local_state = 0;
    func my_func(some_local_state);
    std::thread t(my_func);
    thread_guard g(t); // create our thread guard
    std::cout << "hello world" << std::endl;
    // dont need to join thread, the thread_guard will handle this 
}

int main() {
    f();
    return 0;
}



