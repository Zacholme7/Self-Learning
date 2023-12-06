# 6.2: Creating a process pool
from multiprocessing import Pool

def say_hello(name: str) -> str:
    return f"hi there, {name}"

if __name__ == "__main__":
    # creae a new process pool
    with Pool() as process_pool:
        # run the say_hello function in seperate processes
        hi_jeff = process_pool.apply(say_hello, args = ("Jeff",))
        hi_john = process_pool.apply(say_hello, args = ("John",))
        print(hi_jeff)
        print(hi_john)
