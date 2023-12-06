# 6.4: Process poll executors
import time
from concurrent.futures import ProcessPoolExecutor

def count(count_to: int) -> int:
    start = time.time()
    counter = 0
    while counter < count_to:
        counter = counter + 1
    end = time.time()
    print(f"Finished counting to {count_to} in {end - start}")
    return counter


if __name__ == "__main__":
    # start a pool of process executors
    # this is a context manager
    with ProcessPoolExecutor() as process_pool:
        numbers = [1, 3, 5, 22, 100000000]
        # will map the count function to each number in numbers
        for result in process_pool.map(count, numbers):
            print(result)

        
