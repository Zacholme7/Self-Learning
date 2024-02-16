# 7.7: Using to_thread coroutine
import requests
import asyncio
from util import async_timed

def get_status_code(url: str) -> int:
    response = requests.get(url)
    return response.status_code

@async_timed
async def main():
    urls = ["https://www.example.com" for _ in range(1000)]
    # make a coroutine for each url
    tasks = [asyncio.to_thread(get_status_code, url) for url in urls]
    # await all of our tasks
    results = await asyncio.gather(*tasks)
    print(results)

asyncio.run(main())
