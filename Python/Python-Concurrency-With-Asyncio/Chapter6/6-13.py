# 6.13: Initializing a process pool
from concurrent.futures import ProcessPoolExecutor
import asyncio
from multiprocessing import Value

shared_counter: Value # our shared counter

def init(counter: Value):
    global shared_counter # declare it as global
    shared_counter = counter

def increment():
    # lock and increment
    with shared_counter.get_lock():
        shared_counter.value += 1

async def main():
    counter = Value("d", 0)
    # make the process pool, pass in our init function and our counter
    # this will be called for each process that the process pool creates
    with ProcessPoolExecutor(initializer = init, initargs = (counter,)) as pool:
        await asyncio.get_running_loop().run_in_executor(pool, increment)
        print(counter.value)

if __name__ == "__main__":
    asyncio.run(main())
