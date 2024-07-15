#include <stdexcept>
#include <thread>
#include <iostream>


class ScopedThread {
        // the thread that is moved into this object
        std::thread t; 
public:
        // takes ownership
        explicit ScopedThread(std::thread _t):
                t(std::move(_t)) 
        {
                if (!t.joinable())
                        throw std::logic_error("no thread");
        }

        // destructor
        ~ScopedThread() noexcept {
                // just join upon destruction
                // we know this is valid because we checked if it was joinable upon constructor
                t.join();
        }

        // delete copy constructor and copy assignment constructor
        ScopedThread(ScopedThread const&) = delete;
        ScopedThread& operator=(ScopedThread const&) = delete;
};

int main() {
        ScopedThread st(std::thread([]{
                std::cout << "hello world" << std::endl;
        }));
}