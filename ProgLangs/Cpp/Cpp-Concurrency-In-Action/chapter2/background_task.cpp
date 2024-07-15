#include <iostream>
#include <thread>

class BackgroundTask {
public:
  void operator()() { doSomething(); }

private:
  void doSomething() { std::cout << "in do something\n"; }
};

int main() {
  BackgroundTask task;

  // task is COPIED in, have to make sure the copy behaves the same way
  std::thread backgroundThread(task);
  backgroundThread.join();
}
