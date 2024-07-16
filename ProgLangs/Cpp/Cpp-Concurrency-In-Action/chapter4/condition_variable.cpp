#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>

std::queue<int> queue;
std::mutex mtx;
std::condition_variable cv;
bool finished = false;

void producer() {
  for (int i = 0; i < 10; i++) {
    // simulate some work
    std::this_thread::sleep_for(std::chrono::milliseconds(100));

    // acquire the lock since we are accessing shared data
    std::lock_guard<std::mutex> lock(mtx);
    queue.push(i);
    std::cout << "Produced " << i << std::endl;
    cv.notify_one();
  }
  std::lock_guard<std::mutex> lock(mtx);
  finished = true;
  cv.notify_all();
}

void consumer() {
  while (true) {
    std::unique_lock<std::mutex> lock(mtx); // allows us to lock and unlock, need for the cv
    // wait until the queue is not empty or we have finished
    // will always check the condition with the mutex locked
    // wait is just an optimization over a busy wait
    cv.wait(lock, [] { return !queue.empty() || finished; });

    if (!queue.empty()) {
      int value = queue.front();
      queue.pop();
      std::cout << "Consumed " << value << std::endl;
    } else if (finished) {
      break;
    }
  }
}

int main() {
  std::thread producerThread(producer);
  std::thread consumerThread(consumer);

  producerThread.join();
  consumerThread.join();

  return 0;
}