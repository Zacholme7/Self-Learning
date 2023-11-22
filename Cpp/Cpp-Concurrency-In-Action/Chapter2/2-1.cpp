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
        }
    }

    // just print the number
    void do_something(unsigned i) {
        std::cout << i << std::endl;
    }
};

void oops() {
    int some_local_state = 0;
    func my_func(some_local_state); // has reference to our local state
    std::thread my_thread(my_func); // start a new thread to execute my_func overload
    // dont wait for it to finish
    my_thread.detach();  
    // new thread might still be running here

}

int main() {
    oops();
    return 0;
}
