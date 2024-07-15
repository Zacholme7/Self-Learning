#include <thread>
#include <iostream>
#include <chrono>

void hello() {
        std::cout << "The id of hello is " << std::this_thread::get_id() << std::endl;
        std::this_thread::sleep_for(std::chrono::seconds(1));
}

int main() {
        std::thread t(hello);
        std::cout << "The id of hello is " << t.get_id() << std::endl;
        std::cout << "The id of main is " << std::this_thread::get_id() << std::endl;
        t.join();
        return 0;
}