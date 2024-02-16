"""Creating a multithreaded python application"""
import threading

# function that will be run in the thread
def hello_from_thread():
    print(f"Hello from thread {threading.current_thread().name}")

# make a new thread and then start it
hello_thread = threading.Thread(target = hello_from_thread)
hello_thread.start()

total_threads = threading.active_count()
thread_name = threading.current_thread().name

print(f'Python is currently running {total_threads} thread(s)')
print(f'The current thread is {thread_name}')

# join the two threads together, makes it so that the application does not exit immediately
hello_thread.join()