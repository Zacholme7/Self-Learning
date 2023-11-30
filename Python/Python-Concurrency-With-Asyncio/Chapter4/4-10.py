# 4.10: Examining the default behavior of wait 
import asyncio
import aiohttp
from aiohttp import ClientSession
from util import async_timed, fetch_status


@async_timed
async def main():
    async with aiohttp.ClientSession() as session:
        fetchers = [asyncio.create_task(fetch_status(session, "https://example.com")),
                    asyncio.create_task(fetch_status(session, "https://example.com"))]

        # wait to get the done and pending 
        done, pending = await asyncio.wait(fetchers)
        
        print(f"Done task count: {len(done)}")
        print(f"Pending task count: {len(pending)}")

        # await all the done tasks
        for done_task in done:
            result = await done_task
            print(result)

asyncio.run(main())

