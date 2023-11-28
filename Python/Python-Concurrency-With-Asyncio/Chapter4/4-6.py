# 4.6: Running requests concurrently with gather
import asyncio
import aiohttp
from aiohttp import ClientSession
from util import fetch_status, async_timed

@async_timed
async def main():
    async with aiohttp.ClientSession() as session:
        urls = ["https://example.com" for _ in range(1000)]
        # generate a list of coroutines for each request we want to make
        requests = [fetch_status(session, url) for url in urls]
        # pass our coroutines to gather that will await them all for us
        status_codes = await asyncio.gather(*requests)
        print(status_codes)

asyncio.run(main())
