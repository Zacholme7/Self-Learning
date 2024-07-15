#include <iostream>
#include <mutex>
#include <thread>

std::mutex mutex1, mutex2;

void thread1Function() {
  std::cout << "Thread 1 trying to lock mutex1\n";
  mutex1.lock();
  std::cout << "Thread 1 locked mutex1\n";

  std::this_thread::sleep_for(std::chrono::milliseconds(1000));

  std::cout << "Thread 1 trying to lock mutex2\n";
  mutex2.lock();
  std::cout << "Thread 1 locked mutex2\n";

  mutex2.unlock();
  mutex1.unlock();
}

void thread2Function() {
  std::cout << "Thread 2 trying to lock mutex2\n";
  mutex2.lock();
  std::cout << "Thread 2 locked mutex2\n";

  std::this_thread::sleep_for(std::chrono::milliseconds(1000));

  std::cout << "Thread 2 trying to lock mutex1\n";
  mutex1.lock();
  std::cout << "Thread 2 locked mutex1\n";

  mutex1.unlock();
  mutex2.unlock();
}

int main() {
  std::thread t1(thread1Function);
  std::thread t2(thread2Function);

  t1.join();
  t2.join();

  return 0;
}
