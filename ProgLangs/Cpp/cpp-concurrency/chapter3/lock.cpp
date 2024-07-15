#include <iostream>
#include <mutex>
#include <thread>

std::mutex mutex1, mutex2;

void thread1Func() {
        std::cout << "Thread one trying to lock mutexes\n";
        // locked and will be unlock when lock goes out of scope 
        std::scoped_lock lock(mutex1, mutex2);
        std::cout << "Thread one unlocked mutexes\n";
}

void thread2Func() {
        std::cout << "Thead two trying to lock mutexes\n";
        // lock and then make into lock guard
        std::lock(mutex2, mutex1);
        // std::adopt says the lock_guard should just adopt the state of the mutex
        std::lock_guard lock2(mutex2, std::adopt_lock);
        std::lock_guard lock3(mutex2, std::adopt_lock);
        std::cout << "Thead two unlocked mutexes\n";
}

int main() {
        std::thread t1(thread1Func);
        std::thread t2(thread2Func);
        t1.join(); t2.join();
        return 0;
}