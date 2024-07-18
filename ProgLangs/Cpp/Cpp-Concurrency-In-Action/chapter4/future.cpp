#include <future>
#include <iostream>
#include <thread>

//- future can be used to represent a one off event
//- unique future, 'std::future<>' (unique_ptr)
//- shared future, 'std::shared_future<>' (shared_ptr)
//- <void> used when there is no data
//- 'std::async' to start an asynchronous task
//  - return a 'std::future'
//  - call 'get()' on the future to get the value when it is readyhrono>

// this is working in the background
int foo() {
    // simulate some work
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    return 100;
}

int bar(int tmp) {
    std::cout << tmp << std::endl;
    return 10;
}

int main() {
    std::future<int> res = std::async(foo);
    // work here
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    std::cout << "the answer is " << res.get() << std::endl;

    // can pass arugments to the future too, the same way as with a thread
    std::future<int> barCall = std::async(bar, 10);
    std::cout << "bar returned " << barCall.get() << std::endl;

    // it is up to the impl if std::async starts a new thread or task runs...
    // ...synchronously when future is waited for
    // 'std::launch::deferred': function call defereed until wait/get are called
    // 'std::launch::async': function must be run on its own thread
    // 'std::launch::async | std::launch::deferred' to let the impl choose. this is default
    auto res1 = std::async(std::launch::async, bar, 10); // this is run in a new thread
    auto res2 = std::async(std::launch::deferred, bar, 10); // this is run in wait/get
    res2.wait(); // it is run here

    return 0;
}