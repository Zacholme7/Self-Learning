// 29: maybe_unused attribute
// can just mark things as possibly unused
// may need conditional compilation


int main(int argc, [[maybe_unused]]const char *argv[]) {
  return 0;
}
