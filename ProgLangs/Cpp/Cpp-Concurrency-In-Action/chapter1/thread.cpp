#include <iostream>
#include <thread>

void hello() { std::cout << "hello\n"; }

void world() { std::cout << "world\n"; }

int main() {
  std::thread t(hello);
  std::thread x(world);
  t.join();
  x.join();
}
