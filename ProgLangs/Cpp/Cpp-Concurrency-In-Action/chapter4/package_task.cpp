#include <iostream>
#include <future>
#include <chrono>


int compute_value(int x) {
    std::this_thread::sleep_for(std::chrono::seconds(2));
    return x * x;
}
int main() {
    // 'std::packaged_task<>' ties future to a function or callable object
    // when invoked, it calls the associated function and makes the future ready
    // return types specifies the type of the future that is returned

    // create a new packaged task
    // function that takes an int and returns an int
    std::packaged_task<int(int)> task(compute_value);
    // get the future associated with it
    std::future<int> result = task.get_future();
    // launch the thread
    std::thread task_thread(std::move(task), 5);

    // do something while the task is running
    std::cout << "The result is " << result.get() << std::endl;
    task_thread.join();



    return 0;
}