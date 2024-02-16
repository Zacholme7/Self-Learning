// 9: std::future part 2
#include <random>
#include <algorithm>
#include <set>
#include <iostream>
#include <future>


// number generator in lecture 9
std::set<int> make_sorted_random(const size_t num_elements) {
  std::set<int> retval; 
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<> dis(0, num_elements - 1);
  std::generate_n(std::inserter(retval, retval.end()), num_elements, [&](){ return dis(gen); });
  return retval;
}

int main() {
  // default args are async or deferred
  // if it should launch it or store it deferred
  // .get() blocks until the value is available
  // asynchronously calculate list of numbers then block until it is available

  // now we are launching both and running them
  auto f1 = std::async(std::launch::async, make_sorted_random, 1000000);
  auto f2 = std::async(std::launch::async, make_sorted_random, 1000000);
  return 0;
}
