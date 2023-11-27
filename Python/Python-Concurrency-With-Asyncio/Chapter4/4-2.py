#4.2: Making an aiohttp web request
import asyncio
import aiohttp
from aiohttp import ClientSession
from util import async_timed

@async_timed
async def fetch_status(session: ClientSession, url: str) -> int:
    # send the request and return the status
    async with session.get(url) as result:
        return result.status

@async_timed
async def main():
    # open a new context manager to make a session
    async with aiohttp.ClientSession() as session:
        url = "https://www.example.com"
        # call helper function to send requrest
        status = await fetch_status(session, url)
        print(f"Status for {url} was {status}")

asyncio.run(main())
