# 4.12: Canceling running requests on an exception
# looks what happens when we have a couple of long running web requests
# that wed like to cancel when one coroutine fails immediately with an exception
import aiohttp
import asyncio
import logging
from util import fetch_status, async_timed

@async_timed
async def main():
    async with aiohttp.ClientSession() as session:
        # create the fetchers
        fetchers = [asyncio.create_task(fetch_status(session, "python://bad.com")),
                    asyncio.create_task(fetch_status(session, "http://www.example.com", delay = 3)),
                    asyncio.create_task(fetch_status(session, "https://www.example.com", delay = 3))]

        # get the done and pending tasks
        done, pending = await asyncio.wait(fetchers, return_when = asyncio.FIRST_EXCEPTION)

        print(f"Done task count: {len(done)}")
        print(f"Pending task count: {len(pending)}")
            
        # check on all the done tasks
        for done_task in done:
            if done_task.exception() is None:
                print(done_task.result())
            else:
                logging.error("Request got an exception", exc_info = done_task.exception())

        # cancel all the pending tasks
        for pending_task in pending:
            pending_task.cancel()

asyncio.run(main())

