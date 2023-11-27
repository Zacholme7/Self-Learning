# 4.3: Setting timeouts with aiohttp
import asyncio
import aiohttp
from aiohttp import ClientSession

async def fetch_status(session: ClientSession, url: str) -> int:
    # override timeout for get request to 10ms
    ten_millis = aiohttp.ClientTimeout(total = .15)
    # make the request with a timeout
    async with session.get(url, timeout = ten_millis) as result:
        return result.status

async def main():
    # client session level timeout set to 1 second and connection timeout of 100ms
    session_timeout = aiohttp.ClientTimeout(total = 1, connect = .1)
    # make a session with a timeout
    async with aiohttp.ClientSession(timeout = session_timeout) as session:
        # fetch the status with a timeout
        await fetch_status(session, "https://example.com")

asyncio.run(main())

