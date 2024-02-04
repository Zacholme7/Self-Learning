// 28: C++17 fallthrough attribute

void do_something() {}
void do_something_else() {}

int main(int argc, const char *[]) {
  switch(argc)
  {
    case 1:
      do_something();
      [[fallthrough]]; // tells the compiler we ant it to fall through
    case 2:
      do_something_else();
  }
}
