#include <iostream>
#include <thread>

class thread_guard{
    std::thread &t;
public:
    explicit thread_guard(std::thread& t_):
        t(t_) {}
    ~thread_guard() {
        if (t.joinable()) {
            t.join();
        }
    }
    thread_guard(thread_guard const&) = delete;
    thread_guard& operator=(thread_guard const&)=delete;
};


void hello(int x) {
    std::cout << "hello world\n" << x;
}

int main() {
    std::thread t(hello, 10);
    thread_guard g(t);
}
