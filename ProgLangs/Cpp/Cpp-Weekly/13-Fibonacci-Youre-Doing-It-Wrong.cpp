// 13: Fibaonacci youre doing it wrong
// good to test performance of compiler or language
// implementation with the binet forumula
#include <cmath>
#include <iostream>

constexpr int fib(const int i) {
  constexpr int sqrt_5 = std::sqrt(5);

  if (i == 0)
    return 0;
  if (i == 1)
    return 1;
  return static_cast<int>((std::pow(1 + sqrt_5, i) - std::pow(1 - sqrt_5, i)) /
                          (std::pow(2, i) * sqrt_5));
}

int main() {
  std::cout << fib(45) << '\n';
  return 0;
}
