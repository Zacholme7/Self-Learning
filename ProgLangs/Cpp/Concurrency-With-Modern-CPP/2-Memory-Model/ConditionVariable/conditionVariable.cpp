#include <condition_variable>
#include <iostream>
#include <thread>
#include <vector>

std::vector<int> mySharedWork;
std::mutex mutex_;
std::condition_variable condVar;

bool dataReady{false};

void waitingForWork() {
  std::cout << "Waiting \n";
  std::unique_lock<std::mutex> lck(mutex_);
  condVar.wait(lck, [] {
    return dataReady;
  }); // t1 will wait for the notification from t2
  mySharedWork[1] = 2;
  std::cout << "Work done \n";
}

void setDataReady() {
  mySharedWork = {1, 0, 3}; // prepare the work package
  {
    std::lock_guard<std::mutex> lck(mutex_);
    dataReady = true; // set dataready to be true
  }
  std::cout << "Data prepared\n";
  condVar.notify_one(); // send the notification
}

int main() {
  std::thread t1(waitingForWork);
  std::thread t2(setDataReady);

  t1.join();
  t2.join();

  for (auto v : mySharedWork) {
    std::cout << v << " ";
  }

  std::cout << "\n\n";
}
