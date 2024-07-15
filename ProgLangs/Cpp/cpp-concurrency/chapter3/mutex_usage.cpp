#include <mutex>
#include <vector>
#include <mutex>

std::vector<int> nums;
std::mutex some_mutex;


void add_to_vec(int x) {
        std::lock_guard<std::mutex> guard(some_mutex);
        nums.push_back(x);
}


int main() {
        std::vector<int> nums;
}