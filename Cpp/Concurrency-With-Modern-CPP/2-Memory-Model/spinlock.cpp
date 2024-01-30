#include <atomic>
#include <thread>

class Spinlock {
  // init the atomic flag
  std::atomic_flag flag = ATOMIC_FLAG_INIT;

public:
  // lock the lock, will just continue to spin
  // test and set will set the flag to true upon first call and return a false value
  // the thread will then have the "lock"
  // test_and_set needs to both read and write, these will both be done atomically
  void lock() {
    while (flag.test_and_set());
  }

  // when the thread it done, it will call clear which will set the flag to false
  // this will allow the other thread to access the false value then set it back to true
  void unlock() {
    flag.clear();
  }
};

Spinlock spin;

void workOnResource() {
  spin.lock();
  // shared resources
  spin.unlock();
}


int main() {
  std::thread t(workOnResource);
  std::thread t2(workOnResource);

  t.join();
  t2.join();
  return 0;
}

