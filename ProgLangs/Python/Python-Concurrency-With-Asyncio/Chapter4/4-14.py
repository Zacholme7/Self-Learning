# 4.14: Processing all results as they come in
import asyncio
import aiohttp
from util import async_timed, fetch_status

@async_timed
async def main():
    async with aiohttp.ClientSession() as session:
        url = "https://www.example.com"
        # create all the fetchers
        pending = [
                asyncio.create_task(fetch_status(session, url)),
                asyncio.create_task(fetch_status(session, url)),
                asyncio.create_task(fetch_status(session, url))
                ]

        # process tasks while they are still pending
        while pending:
            # update done and pending sets
            done, pending = await asyncio.wait(pending, return_when = asyncio.FIRST_COMPLETED)
            
            print(f"Done task count: {len(done)}")
            print(f"Pending task count: {len(pending)}")

            for done_task in done:
                print(await done_task)

asyncio.run(main())



