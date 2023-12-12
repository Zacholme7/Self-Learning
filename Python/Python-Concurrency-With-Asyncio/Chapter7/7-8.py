# 7.8: Printing status of requests
import functools
import requests
import asyncio
from concurrent.futures import ThreadPoolExecutor
from threading import Lock
from util import async_timed

counter_lock = Lock()
count: int = 0

def get_status_code(url: str) -> int:
    # declare the global
    global counter
    # send the request
    response = requests.get(url)
    # lock the lock with a context manager
    with counter_lock:
        counter = counter + 1

async def reporter(request_count: int):
    while counter < request_count:
        print(f"Finished {counter}/{request_count} request")
        await asyncio.sleep(.5)

@async_timed
async def main():
    loop = asyncio.get_running_loop()
    with ThreadPoolExecutor() as pool:
        request_count = 200
        urls = ["https://www.example.com" for _ in range(request_count)]
        reporter_task = asyncio.create_task(reporter(request_count))
        tasks = [loop.run_in_executor(pool, functools.partial(get_status_code, url)) for url in urls]
        results = await asyncio.gather(*tasks)
        await reporter_task
        print(results)

asyncio.run(main())