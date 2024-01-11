// 10: Variadic template wrap up 
#include <iostream>
#include <string>
#include <vector>
#include <sstream>

template<typename T>
void printImpl(const T &t) {
  std::cout << t << '\n';
}

int f1() {
  std::cout << "f1\n";
  return 1;
}

int f2() {
  std::cout << "f2\n";
  return 2;
}

template<typename ... T>
void print(const T& ... t) {
 (void)std::initializer_list<int> { (printImpl(t), 0)... };
}

// what is the most succinct way to create a vector of strings from out values?
template<typename ... T>
std::vector<std::string> createStringVec(const T& ... t) {
  std::vector<std::string> retVal;
  std::stringstream ss;
  // reset the string, sstream object to convert to string, push back created value
  // order is guaranteed
  (void)std::initializer_list<int> { (ss.str(""), ss << t, retVal.push_back(ss.str()), 0)... };
  return retVal;

}


int main() {

  // order of evaluation for function calls is unspecified by the standard
  // differnt compilers do it differently
  // with gcc f2 is printed before f1 here...?? demonstrates that compiler can rearrange them
  // with clang f1 is printed before f2
  print(f1(), f2());

  // can use initializer list to make rely on the execution in the order specified
  // things in a comma delimited sequence are also guaranteed to be executed in order
  // these are all guaranteed to be in order
  print("Hello", "world", 1, 2, 3, 5.30f);
  (void)std::initializer_list<int>{ f1(), f2() };
  (f1(), f2());


  for(const auto &s: createStringVec("Hello", "world", 1, 2, 3, 5.30f)) {
    std::cout << s << '\n';
  }
  return 0;

}



