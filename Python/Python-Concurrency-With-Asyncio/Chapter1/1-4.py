import multiprocessing
import os

def hello_from_process():
    print(f"hello from child process {os.getpid()}")

if __name__ == "__main__":
    # make the new process and start it
    hello_process = multiprocessing.Process(target = hello_from_process)
    hello_process.start()

    print(f"hello from the parent process {os.getpid()}")

    # join the process
    hello_process.join()