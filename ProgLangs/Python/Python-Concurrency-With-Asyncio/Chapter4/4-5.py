# 4.5: Using tasks concurrently with a list comprehension
import asyncio
from util import async_timed, delay

@async_timed
async def main() -> None:
    delay_times = [3, 3, 3]
    # make the tasks but dont await them yet
    tasks = [asyncio.create_task(delay(seconds)) for seconds in delay_times]
    # await the tasks so that they can start concurrently
    [await task for task in tasks]

asyncio.run(main())


