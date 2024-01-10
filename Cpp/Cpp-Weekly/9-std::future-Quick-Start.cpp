// 9: std::future Quick Start
#include <random>
#include <algorithm>
#include <set>
#include <iostream>
#include <future>


std::set<int> make_sorted_random(const size_t num_elements) {
  // set to hold all of the values
  std::set<int> retval; 
  // class for generating random values
  std::random_device rd;
  // Mersenne twister random number generator
  // takes seen from std::random_device
  std::mt19937 gen(rd());
  // uniform distribution generator
  std::uniform_int_distribution<> dis(0, num_elements - 1);

  // generate n numbers, takes the set to insert into, the number of elements we want, and the number generator
  std::generate_n(std::inserter(retval, retval.end()), num_elements, [&](){ return dis(gen); });

  return retval;
}

int main() {
  // we want make the function make sorted random be called asynchronously
  // returns a future, a value that is promised to exist at some point in the future
  // get() returns the value that is stored in the future. 
  // async can choose is they run async or deferred
  // can use it to queue up a bunch of things and then delay the execution
  // can use std::launch::async to force them to launch asynchronously
  // makes it parallel, if we run it with /usr/bin/time we see we are using 180% of the cpu
  // can also store and forward exceptions
  // if you work with copies and async you can get good performance boost without worrying about locking
  
  auto f1 = std::async(std::launch::async, make_sorted_random, 1000000);
  auto f2 = std::async(std::launch::async, make_sorted_random, 1000000);

  std::cout << f1.get().size() << " " << f2.get().size() << '\n';

  return 0;
}
