import asyncio
from aiohttp import ClientSession

async def fetch_status(session: ClientSession, url: str) -> int:
    # send the request and return the status
    async with session.get(url) as result:
        return result.status
