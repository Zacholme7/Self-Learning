// #1
// think of function template as looking like this
// template<typename T>
// void f(ParamType param)

// Case 1
template <typename T> void a(T &param);

template <typename T> void b(T *param);

// Case 2
template <typename T> void c(T &&param);

// Case 3
template <typename T> void d(T param);

void someFunc(int, double); // type is void(int, double)

int main() {
  // #1
  // call to a funciton template
  // f(expr) -> call to f with some expression

  // Case 1
  int x = 27;        // x is an int
  const int cx = x;  // cs is a const int
  const int &rx = x; // rx is a reference to x as a const int
  a(x);              // T is an int, params type is int&
  a(cx);             // T is a const int, params type is const int&
  a(rx);             // T is a const int, params type is a const int&

  const int *px = &x; // px is a ptr to x as a const int
  b(&x);              // T is a int, param is int*
  b(px);              // T is a const int, param is const int*

  // Case 2
  c(x);  // x is a lvalue, so T is a int& and paramtype is an int&
  c(cx); // cx is a lvalue, so T is a const int& and paramtype is const int&
  c(rx); // rx is a lvalue, so T is const int& and paramtype is const int&
  c(27); // 27 is a rvalue, so T is an int and paramtype is a int&&

  // Case 3
  // the value is copied in
  d(x);  // T is a int, paramtype is an int
  d(cx); // T is an int, paramtype is an int
  d(rx); //  T is an int, paramtype is an int

  // Array arguments
  const char name[] = "hello world"; // name: const char[13]
  const char *ptrToName = name; // array decays to pointer
  d(name); // T is a const char *, param is a 
  a(name); // T is const char(&f[13]
  
  // Function arguments
  d(someFunc); // param deduced as ptr-to-func of type void(*)(int, double). decays into pointer
  a(someFunc); // param deduced as ref-to-func of type void(&)(int, double). does not decay to pointer

  return 0;
}
