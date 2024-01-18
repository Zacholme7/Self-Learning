// 17: std::invoke
// uniform iterface for calling callable things
// gain not having to know things such as how to invoke a member function pointer
#include <functional>

struct S {
  int j = 5;
  int do_something(const int i) {
    return j + i;
  }
  int do_something2(const int i) {
    return j + i;
  }
};


int do_something(const int i) {
  return 5 + i;
}

int main() {
  std::invoke(&do_something, 5);

  S s;
  s.do_something(3);

  // complicated....
  auto fp = &S::do_something;
  (s.*fp)(3); // call the do something
  int (S::*fp2)(int) = nullptr;
  if (true) {
    fp2 = &S::do_something2;
  } else {
    fp2 = &S::do_something;
  }

  // instead
  // universal interface to call anything that is callable
  std::invoke(&S::do_something, s, 2);
  


  return 0;
}
