// 18: constexpr if statements
// allows you to choose at compile time one or more different branches to take
// in your code

#include <iostream>
#include <type_traits>

template <typename T>
auto printTypeInfo(const T &t) {
  // branching at compile time
  // function body just compiles down to the inner if for the corresponding type
  if constexpr (std::is_integral<T>::value && !std::is_same<bool, T>::value) {
    return t + 1;
  } else if constexpr (std::is_floating_point<T>::value) {
    return t + 0.1;
  } else {
    return t;
  }
}

int main() { 
  std::cout << printTypeInfo(5) << '\n';
  std::cout << printTypeInfo(5.1) << '\n';
  std::cout << printTypeInfo(true) << '\n';
  std::cout << printTypeInfo("hello") << '\n';
}
