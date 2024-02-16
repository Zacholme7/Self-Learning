# 4.7: Awaitables finishing out of order
import asyncio
from util import delay

async def main():
    results = await asyncio.gather(delay(3), delay(1))
    # will print [3, 1] since it says in the same order
    print(results)

asyncio.run(main())
