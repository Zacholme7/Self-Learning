#include <thread>
#include <iostream>

void modify(int &x) {
    x = 10;
}

int main() {
    int x = 20;
    std::cout << "the value of x is " << x << std::endl;
    // std::ref is able to be copied here, wrapper around the ref
    std::thread t(modify, std::ref(x));
    t.join();
    std::cout << "the value of x is " << x << std::endl;
    return 0;
}
