#include <future>
#include <iostream>
#include <chrono>

// this is working in the background
int foo() {
    // simulate some work
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    return 100;
}

int main() {
    std::future<int> res = std::async(foo);
    // work here
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    std::cout << "the answer is " << res.get() << std::endl;
    return 0;
}