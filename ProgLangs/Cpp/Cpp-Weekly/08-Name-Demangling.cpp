// 8: Name demangling
// "nm a.out" gives all the names that have been mangled by the compiler
// doesnt mean much to us but means stuff to the compiler
// "nm a.out | c++filt -t" to demangle all the type names
// all the names can quickly get out of hand
// What if we want to get the demangled name at runtime?
// Absolute no idea why this is useful but it is kinda neat

#include <string>
#include <vector>
#include <iostream>
#include <typeinfo>
#include <cxxabi.h>

struct MyClass {
  std::vector<std::string> m_data;
};

std::string demangle(const char *mangled_name) {
  int status;
  char * demangledName = abi::__cxa_demangle(mangled_name, nullptr, 0, &status);
  if (status == 0) {
    std::string retval(demangledName);
    free(demangledName);
    return retval;
  } else {
    throw std::runtime_error("Error demangling");
  }
}

int main() {
  MyClass myclass;

  int status;
  std::cout << demangle(typeid(&MyClass::m_data).name()) << '\n';
  return 0;
}
