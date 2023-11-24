// 2.9: a naive parallel version of std::accumulate

#include <iostream>
#include <thread>


template<typename Iterator, typename T>
struct accumulate_block {
    void operator() (Iterator first, Iterator last, T& result) {
        result = std::accumulate(first, last, result);
    }
};

template<typename Iterator, typename T>
T parallel_accumulate(Iterator first, Iterator last, T init) {
    // distance between the interators
    unsigned long const length = std::distance(first, last);

    // they are the same, no distance
    if(!length)
        return init;

    // min elements per thread
    unsigned long const min_per_thread = 25; 

    // the max threads to achieve min_per_thread
    unsigned long const max_threads = (length + min_per_thread - 1) / min_per_thread;

    // the number of hardware threads
    unsigned long const hardware_threads = std::thread::hardware_concurrency();

    // determine the number of threads that we should use
    unsigned long const num_threads = std::min(hardware_threads != 0 ? hardware_threads:2, max_threads);

    // how much should be allocated to each thread
    unsigned long const block_size = length / num_threads;

    // vector to hold the results of each thread
    std::vector<T> results(num_threads);

    // vector to hold all of the threads
    std::vector<std::thread> threads(num_threads - 1);

    Iterator block_start = first;

    // assign all of the worl
    for(unsigned long i = 0; i < (num_threads - 1); i++) {
        Iterator block_end = block_start;
        std::advance(block_end, block_size);
        threads[i] = std::thread(accumulate_block<Iterator,T>(), block_start, block_end, std::ref(results[i]));
        block_start = block_end;
    }

    // total the last thread
    accumulate_block<Iterator, T>() (block_start, last, results[num_threads - 1]);

    // join all the threads
    for(auto& entry: threads)
        entry.join();

    
    // return the total
    return std::accumulate(results.begin(), results.end(), init);
}
