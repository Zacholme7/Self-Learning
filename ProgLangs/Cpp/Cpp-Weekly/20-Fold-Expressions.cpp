// link: https://www.youtube.com/watch?v=nhk8pF_SlTk&list=PLs3KjaCtOwSZ2tbuV1hx8Xz-rFZTan2J1&index=21
// can be expanded on left, right, or middle depending where the expansion is

#include <iostream>

// expansion on the right
template<typename ... T>
auto sum(T ... t) {
  return ( t + ...);
}

// expansion on the left
template<typename ... T>
auto div(T ... t) {
  return ( ... / t );
}

// showing off some possibilities
template<typename ... T>
auto avg(T ... t) {
  return (t + ...) / sizeof...(t);
}

int main() {
  std::cout << sum(1, 2, 3, 4) << '\n';
  std::cout << div(1, 2, 3, 4) << '\n';
  std::cout << avg(1, 2, 3, 4) << '\n';
  return 0;
}
