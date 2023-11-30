# 4.16: Canceling a slow request
import asyncio
import aiohttp
from util import fetch_status

async def main():
    async with aiohttp.ClientSession() as session:
        # the requests that we want to make
        api_a = fetch_status(session, "https://www.example.com")
        api_b = fetch_status(session, "https://example.com", delay = 2)

        done, pending = await asyncio.wait([api_a, api_b], timeout = 1)

        for task in pending:
            # if b is not done, cancel it
            if task is api_b:
                print("API B too slow, cancelling")
                task.cancel()

asyncio.run(main())

# result, dont see the print statement about api b
# the coroutines are automatically wrapped in tasks
# cannot do a comparison because of this
