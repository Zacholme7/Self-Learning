# 6.2: Creating a process pool
from multiprocessing import Pool

def say_hello(name: str) -> str:
    return f"hi there, {name}"

if __name__ == "__main__":
    # creae a new process pool
    with Pool() as process_pool:
        # apply_async so we are not blocking, start instantly in each process
        hi_jeff = process_pool.apply_async(say_hello, args = ("Jeff",))
        hi_john = process_pool.apply_async(say_hello, args = ("John",))
        # when we callget, parent process will block until each returns a value
        print(hi_jeff.get())
        print(hi_john.get())
