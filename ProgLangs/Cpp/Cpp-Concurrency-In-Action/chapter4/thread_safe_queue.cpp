#include <thread>
#include <iostream>
#include <condition_variable>
#include <mutex>
#include <chrono>
#include <queue>

template<typename T>
class threadsafe_queue {
private:
    // mutex defines as mutable, so we can lock in const situations
    mutable std::mutex mut;
    // the data
    std::queue<T> data_queue;
    // synchronization
    std::condition_variable data_cond;
public:
    // constructor
    threadsafe_queue() {}

    // constructor from another queue
    threadsafe_queue(threadsafe_queue const& other) {
        // lock the other mutex since we are accessing shared data
        std::lock_guard<std::mutex> lk(other.mut);
        data_queue = other.data_queue;
    }

    // push a new value
    void push(T new_value) {
        std::lock_guard<std::mutex> lk(mut); // get the lock
        data_queue.push(new_value); // can now access data
        data_cond.notify_one(); // signal that there is new data
    }

    // pop an element from the queue
    std::shared_ptr<T> pop() {
        std::unique_lock<std::mutex> lk(mut); // want to lock and unlock
        data_cond.wait(lk, [this]{ return !data_queue.empty();});
        std::shared_ptr<T> res = std::make_shared<T>(std::move(data_queue.front()));
        data_queue.pop();
        return res;
    }

    // check if it is empty
    bool empty() const {
        // this funciton is const but we can still lock since mut is mutable
        std::lock_guard<std::mutex> lk(mut);
        return data_queue.empty();
    }
};


void producer(threadsafe_queue<int> &q) {
    for(int i = 0; i < 100; i++) {
        std::cout << "pushing an element onto the queue\n";
        // simulate some work
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
        q.push(i);
        std::cout << "pushed an element onto the queue\n";

    }
}

void consumer(threadsafe_queue<int> &q) {
    for(int i = 0; i < 100; i++) {
        std::cout << "waiting for a value\n";
        std::shared_ptr<int> res = q.pop();
        std::cout << "got the value " << res << std::endl;
    }
}


int main() {
    threadsafe_queue q = threadsafe_queue<int>{};
    std::thread p(producer, std::ref(q));
    std::thread c(consumer, std::ref(q));

    p.join();
    c.join();
    return 0;
}