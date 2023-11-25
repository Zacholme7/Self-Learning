// 3.1: Protecting a list with a mutex
#include <algorithm>
#include <list>
#include <iostream>
#include <thread>
#include <mutex>

// list global variable
std::list<int> some_list; 
// global mutex
std::mutex some_mutex;

// function to add a new value to the list
void add_to_list(int new_value) {
    // lock guard that locks the mutex
    //
    std::lock_guard guard(some_mutex);
    // add the value to the list
    some_list.push_back(new_value);
    // end of function which will drop lock_guard, unlocking the mutex
}

// searches for a value in the list
bool list_contains(int value_to_find) {
    // lock guard that locks the mutex
    // can remove the type due to c++17 class template argument deduction
    std::lock_guard<std::mutex> guard(some_mutex);
    // try to find the element
    return std::find(some_list.begin(), some_list.end(), value_to_find) != some_list.end();
    // end of function which will drop lock_guard, unlocking the mutex
}

int main() {
    add_to_list(10);
    list_contains(10);
    return 0;
}
