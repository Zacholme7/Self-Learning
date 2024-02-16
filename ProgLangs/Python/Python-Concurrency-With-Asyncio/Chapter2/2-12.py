import asyncio
from util import delay

async def main():
    # coroutine that will run for 2 seconds
    delay_task = asyncio.create_task(delay(2))
    try:
        # wait the task and time it out after 1 second, should throw an exception since task takes 2 seconds
        result = await asyncio.wait_for(delay_task, timeout = 1)
        print(result)
    except asyncio.exceptions.TimeoutError:
        print("Got a timeout")
        print(f"Was the task cancelled? {delay_task.cancelled()}")

asyncio.run(main())