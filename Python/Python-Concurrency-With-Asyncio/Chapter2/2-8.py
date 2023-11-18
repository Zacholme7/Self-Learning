import asyncio
from util import delay

async def main():
    # create a tasks that we need to await
    sleep_for_three = asyncio.create_task(delay(3))

    # this will print right away
    print(type(sleep_for_three))

    # now await it which will do the "work"
    result = await sleep_for_three
    print(result)

asyncio.run(main())
