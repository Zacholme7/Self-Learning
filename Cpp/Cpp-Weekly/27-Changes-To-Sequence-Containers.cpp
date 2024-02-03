// 27: changes to sequence containers
// containers that have data that can be accessed sequentially

// emplace back. construct new type and put it where it needs to go in memory
// - calling constructor

#include <vector>

int main() {

  std::vector<std::vector<int>> v;
  // emplace it and then return it
  auto &new_vec = v.emplace_back(3, 1);
  new_vec.push_back(4);
  return 0;
}
