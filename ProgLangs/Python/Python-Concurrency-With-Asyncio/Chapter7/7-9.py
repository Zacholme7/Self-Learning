# 7.9: Recursion with locks
from threading import RLock, Thread
from typing import List

list_lock = RLock()

def sum_list(int_list: List[int]) -> int:
    print("Waiting to acquire lock...")
    with list_lock:
        print("Acquired lock")
        if len(int_list) == 0:
            print("Finished summing")
            return 0
        else:
            # decouple the list
            head, *tail = int_list
            print("summing the rest of the list")
            return head + sum_list(tail)

thread = Thread(target = sum_list, args = ([1, 2, 3, 4],))
thread.start()
thread.join()
