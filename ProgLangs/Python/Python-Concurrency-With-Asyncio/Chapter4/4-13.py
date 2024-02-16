# 4.13: Process as they complete
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
                asyncio.create_task(fetch_status(session, url))
                ]

        # get the done and pending tasks with first completed
        # will return as soon as any of the request completes
        done, pending = await asyncio.wait(fetchers, return_when = asyncio.FIRST_COMPLETED)

        print(f"Done task count: {len(done)}")
        print(f"Pending task count: {len(pending)}")

        for done_task in done:
            print(await done_task)

asyncio.run(main())



