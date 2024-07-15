#include <thread>
#include <iostream>

int main() {
        int num = std::thread::hardware_concurrency();
        std::cout << "There are " << num << "hardware threads\n";
        return 0;
}