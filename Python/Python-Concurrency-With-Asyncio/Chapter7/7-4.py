# 7.4: Running requests with a thread pool
import time
import requests
from concurrent.futures import ThreadPoolExecutor

def get_status_code(url: str) -> int:
    response = requests.get(url)
    return response.status_code

start = time.time()

# make thread pool executor with 1000 threads
with ThreadPoolExecutor(max_workers = 1000) as pool:
    urls = ["https://www.example.com" for _ in range(1000)]
    # map a thread to all of the functions
    results = pool.map(get_status_code, urls)
    for result in results:
        print(result)

end = time.time()

print(f"Finished requests in {end - start:.4f} seconds")
