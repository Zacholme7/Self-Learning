// 14: standard library gems next and exchange
#include <vector>
#include <list>
#include <iostream>
#include <algorithm>

int main() {
  // next will give you the next forward iterator
  // makes a copy of the iterator and increments it to the next value

  std::list<int> v {3, 2, 3, 4, 5, 6, 7};
  std::cout << std::is_sorted(std::next(v.begin()), v.end()) << '\n';

  // exchange will replaces value with new value and returns the old value
  // attemps to make sure that no copies happen
  for( int i = 0; i < 100; i++) {
    //last = std::exchange(i, new_value);
    //last = i;
    //i = new_value
  }

}
