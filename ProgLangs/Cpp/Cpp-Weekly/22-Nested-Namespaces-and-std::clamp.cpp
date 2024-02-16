// 21: nested namespaces and c++ clamp
#include <iostream>
#include <algorithm>

// can specify nested namespaces all at once
namespace org::group::project::internal {
  struct myStruct {};
}

int main(const int argc, const char *[]) {

  // access the nested namespace
  org::group::project::internal::myStruct s;

  // clamp a number into a range
  std::cout << std::clamp(argc, 2, 5) << '\n';

  return 0;
}
