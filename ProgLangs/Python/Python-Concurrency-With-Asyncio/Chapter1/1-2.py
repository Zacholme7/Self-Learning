"""Processes and threads in a simple Python application"""
import os
import threading

# see what the main process id is
print(f"Python process running with process id {os.getpid()}")

# get the total number of threads and the current threads name
total_threads = threading.active_count()
thread_name = threading.current_thread().name

print(f"Python is current running {total_threads} thread(s)")
print(f"The current thread is {thread_name}")



