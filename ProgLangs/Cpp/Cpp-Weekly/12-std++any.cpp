// 12: std::any
// typesafe container for a value of anything
//
#include <any>
#include <vector>
#include <iostream>

struct S {
  S(const S &s) = default;
  S() = default;
};

int main() {
  // heterogenous array
  std::vector<std::any> v{std::any(5), std::any(3.4), std::any("hello world"), std::any(S())};

  std::cout << std::any_cast<int>(v[0]) << '\n';
  std::cout << v[1].type().name() << '\n';

}
