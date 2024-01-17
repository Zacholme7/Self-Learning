// 16: Avoid std::bind
#include <iostream>

void print(int i, const std::string &s) { std::cout << i << ' ' << s << '\n'; }

int main() {
  // bind is a lot of compile and runtime overhead
  // just use a lambda instead of bind is the takeaway

  // this operates the same as a bind with less compile time and faster
  int i = 5;
  const auto f = [](auto &&arg1, auto &&arg2) { print(arg2, arg1); };

  for (int j = 0; j < 1000000; j++) {
    f("hello", j);
  }

  return 0;
}
