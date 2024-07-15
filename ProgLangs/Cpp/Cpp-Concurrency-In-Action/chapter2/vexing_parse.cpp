#include <iostream>
#include <thread>

class BackgroundTask {
public:
  void operator()() const { std::cout << "Background task is running.\n"; }
};

int main() {
  // this line will be interpreted as a function declaration
  // - my_thread function taking single parameter of type pointer
  // to a function taking no parameters and returning background task
  // object
  // std::thread my_thread(Backgroundtask());

  // correct ways
  // Solution 1: Named variable
  BackgroundTask task;
  std::thread correct_thread1(task);

  // Solution 2: Extra parentheses
  std::thread correct_thread2((BackgroundTask()));

  // Solution 3: Uniform initialization syntax
  std::thread correct_thread3{BackgroundTask()};

  // Join the threads to prevent crashes
  correct_thread1.join();
  correct_thread2.join();
  correct_thread3.join();
}
