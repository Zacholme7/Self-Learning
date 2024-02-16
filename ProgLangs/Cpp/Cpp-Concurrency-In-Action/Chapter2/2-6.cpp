// 2.6: Scoped thread and example usage
// when the scoped_thread object goes out of scope it is joined

#include <iostream>
#include <thread>


class scoped_thread {
    std::thread t; // local thread
public:
    // constructor that moves the thread into this class
    explicit scoped_thread(std::thread t_):
        t(std::move(t_)) {
            if(!t.joinable()) {
                throw std::logic_error("No thread");
            }
        }
    
    // destructor to handle up cleanup
    ~scoped_thread() {
        t.join(); // just join the thread
    }

    // delete copy and copy assignment
    scoped_thread(scoped_thread const&) = delete;
    scoped_thread &operator=(scoped_thread const&) = delete;
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
    int some_local_state;
    scoped_thread t{std::thread(func(some_local_state))};
    std::cout << "in function f" << std::endl;
}

int main() {
    f();
    return 0;
}
