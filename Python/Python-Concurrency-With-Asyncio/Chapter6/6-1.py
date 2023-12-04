# 6.1: Two parallel processes with multiprocessing
import time
from multiprocessing import Process

def count(count_to: int) -> int:
    start = time.time()
    counter = 0
    while counter < count_to:
        counter = counter + 1
    end = time.time()
    print(f"Finished counting to {count_to} in {end - start}")

if __name__ == "__main__":
    start_time = time.time()

    # create a process to run the countdown function
    to_one_hundred_million = Process(target = count, args = (100000000,))
    to_two_hundred_million = Process(target = count, args = (200000000,))

    # start the process, this method runs instantly
    to_one_hundred_million.start()  
    to_two_hundred_million.start()

    # wait for the processes to join, block until the process is done
    to_one_hundred_million.join()
    to_two_hundred_million.join()

    end_time = time.time()
    print(f"Completed in {end_time - start_time}")

