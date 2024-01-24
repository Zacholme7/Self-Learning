// Item 2: Understand auto type deduction


int main() {
  // the type specifier for x is simply auto by itself
  auto x = 27;

  // type specifier is const auto
  const auto cx = x;
  const auto &rx = x;
  
  return 0;
}
