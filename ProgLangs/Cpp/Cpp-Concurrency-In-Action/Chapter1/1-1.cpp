#include <iostream>
#include <thread>

void say_hello() {
    std::cout << "hello" << std::endl;
}

int main() {
    std::thread t(say_hello);
    t.join();
    return 0;
}
