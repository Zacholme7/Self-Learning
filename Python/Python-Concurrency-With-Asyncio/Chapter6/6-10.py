# 6.10: Shared values and arrays
from multiprocessing import Process, Value, Array

# just increment the shared value
def increment_value(shared_int: Value):
    shared_int.value = shared_int.value + 1

# increment every value in the array
def increment_array(shared_array: Array):
    for index, integer in enumerate(shared_array):
        shared_array[index] = integer + 1

if __name__ == "__main__":
    # create our shared memory
    integer = Value("i", 0)
    integer_array = Array("i", [0, 0])

    # create two processes
    procs = [Process(target = increment_value, args = (integer,)),
             Process(target = increment_array, args = (integer_array,))]

    # start and join the processes
    [p.start() for p in procs]
    [p.join() for p in procs]

    print(integer.value)
    print(integer_array[:])


