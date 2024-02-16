// 25: Aggregate initialization

struct A {
  int a;
};

struct B : A {
  int b;
  int c;
};


int main() {
  // can use to initialize both the child and the parent class
  B b{{1}, 2, 3};
}
