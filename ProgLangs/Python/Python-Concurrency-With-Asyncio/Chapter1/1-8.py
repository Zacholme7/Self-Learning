import time
import threading
import requests

def read_example():
    response = requests.get("https://www.google.com")
    print(response.status_code)

# create the threads
thread_1 = threading.Thread(target = read_example)
thread_2 = threading.Thread(target = read_example)

# time the threads
thread_start = time.time()

# start then join
thread_1.start()
thread_2.start()
print("All threads running")
thread_1.join()
thread_2.join()

thread_end = time.time()

print(f"Running with threads took {thread_end - thread_start:.4f} seconds")

#All threads running
#200
#200
#Running with threads took 0.1640 seconds