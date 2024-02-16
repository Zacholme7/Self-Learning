import time
# fibbonacci without any threading

def print_fib(number: int) -> None:
    def fib(n: int) -> int:
        if n == 1:
            return 0
        elif n == 2:
            return 1
        else:
            return fib(n - 1) + fib(n - 2)
    print(f"fib({number}) is {fib(number)}")

def fibs_no_threading():
    print_fib(40)
    print_fib(41)

# time the function
start = time.time()
fibs_no_threading()
end = time.time()

print(f"Completed in {end - start:.4f} seconds")

#fib(40) is 63245986
#fib(41) is 102334155
#Completed in 31.6506 seconds