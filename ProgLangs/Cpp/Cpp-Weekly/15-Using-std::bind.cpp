// using std::bind
// takes a callable object and a list of args that want to bind to it
// it binds parameters to functions
// not very lightweight and have pretty big impacts on compile time
// can be used to make a bunch of differenty nifty adaptor

#include <functional>
#include <iostream>

void print(const int &i) { std::cout << i << '\n'; }

void print2(int i, const std::string &s) { std::cout << i << ' ' << s << '\n'; }

template<typename T>
void print3(T i, const std::string &s) { std::cout << i << ' ' << s << '\n'; }

int main() {
  // bind the print function to 5
  int i = 5;
  const auto f = std::bind(&print, i);

  // we will always see 5 printed now
  f();
  i = 6; // still only get 5 because bind makes a copy of the args passed in
  f();

  // if we want value to actually track
  int j = 5;
  const auto e = std::bind(&print, std::ref(j));
  e();
  j = 6; // now we will see 6 printed since we use a standard reference
  e();

  // can still pass in a parameter
  int k = 10;
  const auto h = std::bind(&print2, std::ref(k), std::placeholders::_1);
  h("hello"); // will print 10 then hello

  // can swap the parameters
  int l = 10;
  const auto m = std::bind(&print2, std::placeholders::_2, std::placeholders::_1);
  m("hello", 1); // will print 10 then hello

  // also will swallow all extra args, maybe this was just easier to write??
  m("hello", 1, 2, 3, 4);

  // have to specify type for template function
  const auto q = std::bind(&print3<int>, 10, "hello");

  return 0;
}
