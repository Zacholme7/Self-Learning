// Returning a std::thread from a function

#include <iostream>
#include <thread>

// function that returns a thread
std::thread f() {
    void some_function();
    return std::thread(some_function); // return a thread associated with some_function
}

std::thread g() {
    void some_other_function(int);
    std::thread t(some_other_function, 42);
    return t
}

// can also accept a thread as a parameter
void h(std::thread t);
void i() {
    void diff_function();
    // call h directly with a new thread
    h(std::thread(diff_function));

    // create a thread and move it into the function
    std::thread t(diff_function);
    h(std::move(t))
}

int main() {
    return 0;
}
