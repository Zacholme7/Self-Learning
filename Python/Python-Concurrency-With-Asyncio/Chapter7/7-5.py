# 7.5: Using a thread pool executor with asyncio
import functools
import asyncio
import requests
from concurrent.futures import ThreadPoolExecutor
from util import async_timed

def get_status_code(url: str) -> int:
    response = requests.get(url)
    return response.status_code

@async_timed
async def main():
    # get the event loop
    loop = asyncio.get_running_loop()
    # make our threadpool
    with ThreadPoolExecutor() as pool:
        # create all of the urls
        urls = ["https://www.example.com" for _ in range(1000)]
        # create all of our tasks
        tasks = [loop.run_in_executor(pool, functools.partial(get_status_code, url)) for url in urls]
        # await all of the tasks
        results = await asyncio.gather(*tasks)
        print(results)

asyncio.run(main())


