# 7.13: The stress test class
import asyncio
from concurrent.futures import Future
from asyncio import AbstractEventLoop
from typing import Callable, Optional
from aiohttp import ClientSession

class StressTest:
    def __init__(self, loop: AbstractEventLoop, url: str, total_requests: int, callback: Callable[[int, int], None]):
        self._completed_requests: int = 0 # the number of requests that have completed
        self._load_test_future: Optional[Future] = None # the future
        self._loop = loop # the event loop
        self._url = url # the url we want to test
        self._total_requests = total_requests # the total amount of requests that we want to send
        self._callback = callback # the callback functoin
        self._refresh_rate =  total_requests // 100 # referesh rate

    # start the test
    def start(self):
        # get the future for the coroutine on the loop
        future = asyncio.run_coroutine_threadsafe(self._make_requests(), self._loop)
        # set the future
        self._load_test_future = future

    # cancel the sest
    def cancel(self):
        # if we have a funture (we are running)
        if self._load_test_future:
            # cancel the future
            self._loop.call_soon_threadsafe(self._load_test_future.cancel)

    # function to get the url
    async def _get_url(self, session: ClientSession, url: str):
        try:
            # get the url
            await session.get(url)
        except Exception as e:
            print(e)
        
        # if we are done, call the callback
        if self._completed_requests % self._refresh_rate == 0 or self._completed_requests == self._total_requests:
            self._callback(self._completed_requests, self._total_requests)

    # make all the requests
    async def _make_requests(self):
        async wtih ClientSession() as session:
            reqs = [self._get_url(session, self._url) for _ in range(self._total_requests)]
            await asyncio.gather(*reqs)
