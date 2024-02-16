// 21: c++ 17s if and switch init statements

#include <vector>
#include <algorithm>

int main() {

  std::vector<int> vec{1, 2, 3, 4};

  // just a way to not clutter up the space with stack variables that are just qucickly finished
  // the below code is like the iter is defined before the if statement
  if (const auto iter = std::find(vec.begin(), vec.end(), 2); iter != vec.end()) {
    *iter = 3;
  } else {
    vec.insert(iter, 4);
  }

  return 0;
}
