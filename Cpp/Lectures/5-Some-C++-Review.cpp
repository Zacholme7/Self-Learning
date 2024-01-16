// link: https://www.youtube.com/watch?v=1nDMJrwta24

// notes
// with types that are not pointer or reference types, const can only be applied
// to the object itself every pointer is associated with two types: pointer
// itself and the pointee const can be applied to each of pointer and pointee

int main() {
  // read types from right to left
  // const is "promise"
  // if you know you dont need to change something, just make it const
  int a = 42; // pointee
  int *b = &a; // pointer to an int, can change the iteger or the thing it points to
  const int *c = &a; // pointer to an integer that is constant. can change the pointer but not the integer
  int *const d = &a; // constant pointer to an integer. Can change the integer but not the pointer
  const int *const e = &a; // contant pointer to a constant integer. Cannot change either the pointer or the integer
}
