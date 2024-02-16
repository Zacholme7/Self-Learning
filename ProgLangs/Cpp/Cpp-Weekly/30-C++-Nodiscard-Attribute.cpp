// 26: nodiscard attribute
// link: https://www.youtube.com/watch?v=l_5PF3GQLKc&list=PLs3KjaCtOwSZ2tbuV1hx8Xz-rFZTan2J1&index=32

struct [[nodiscard]] MyError {

};

// the return value from this function is important, it should not be discarded
[[nodiscard]] MyError something() {
  return {};
}

[[nodiscard]] int something2() {
  return 1;
}

int main() {
  // have to work hard to get the compiler to really ignore the value
  something();
  return 0;
}
