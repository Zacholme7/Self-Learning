# 4.14: Using timeouts with wait
import asyncio
import aiohttp
from util import async_timed, fetch_status

@async_timed
async def main():
    async with aiohttp.ClientSession() as session:
        url = "https://www.example.com"
        # create all the fetchers
        fetchers = [
                asyncio.create_task(fetch_status(session, url)),
                asyncio.create_task(fetch_status(session, url)),
                asyncio.create_task(fetch_status(session, url, delay = 3))
                ]

        # will run as many tasks as it can in 1 second
        done, pending = await asyncio.wait(fetchers, timeout = 1)

        print(f"Done task count: {len(done)}")
        print(f"Pending task count: {len(pending)}")

        for done_task in done:
            print(await done_task)

asyncio.run(main())



