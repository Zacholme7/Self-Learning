# 6.11: Incrementing a shared counter in parallel
from multiprocessing import Process, Value

def increment_value(shared_int: Value):
    shared_int.value = shared_int.value + 1

if __name__ == "__main__":
    for _ in range(100):
        integer = Value("i", 0)

        # create two processes
        procs = [Process(target = increment_value, args = (integer,)),
                 Process(target = increment_value, args = (integer,))]

        # start and join all the processes
        [p.start() for p in procs]
        [p.join() for p in procs]
        print(integer.value)
        assert(integer.value == 2)

