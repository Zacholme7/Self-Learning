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
  
  // for reference
  // const qualifier can only be applied to referee
  int j = 10;
  const int &ci = j; // ci is a reference, j, is refree, refree is no modifiable, cant change j by changing ci, can still change j itself 
  // int &cont r = j; ERROR reference itself cannot be specified as a const
  int &test = j;

  // constexpr: qualifer indiciates object has value that is constant expression
  // - can be evaluated at compile time (so constant I can have it at compile time)
  // constexpr implies const (converse not necessarily true)
  // thing on the right hand side needs to be something that you can compute at compile time
  constexpr int x = 42; // x as a constant expression with type const int and value 41
  // int &x2 = x; error: x is a const, x2 is not const
  // int *p2 = &x; error: x is a const, *p2 is not const
  
  // const qualifier can be used in funcion decl to make promise about what non local objects will not be modified by function
  // if pointee is const, function promise not to change the pointeee
  // if referee is const, fucntion promise not to change refree
  // if func wil never modify pointee/referee associated wtih func, param type should be made pointer/reference to const obj
  // if not const, assumption is that you plan to change it
  return 0;
}
