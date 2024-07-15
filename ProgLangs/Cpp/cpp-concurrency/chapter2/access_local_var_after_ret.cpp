#include <iostream>
#include <thread>
#include <chrono>


void dangerousFunction() {
    int localVar = 32;

    // capture the localVar
    auto lambda = [&localVar]() {
        // simulate some work
        std::this_thread::sleep_for(std::chrono::seconds(2));
          // Attempt to access localVar after potentiallyDangerousFunction has returned
        std::cout << "Thread: Value of localVar is " << localVar << std::endl;
        std::cout << "Thread: Address of localVar is " << &localVar << std::endl;
    };

    std::thread t(lambda);
    t.detach();

    // localvar goes out of scope
}


int main() {
    dangerousFunction();
    std::this_thread::sleep_for(std::chrono::seconds(5));
    return 0;
}
