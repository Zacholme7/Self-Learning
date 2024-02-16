import asyncio
from util import delay

async def main():
    # create a task to wrap coroutine
    task = asyncio.create_task(delay(10))

    try:
        # shield prevents task from being cancelled althought it is supposed to cancel after 5 seconds
        result = await asyncio.wait_for(asyncio.shield(task), 5)
        print(result)
    except asyncio.exceptions.TimeoutError:
        print("Task took longer than five seconds, it will finish soon")
        # await the task so that it will finish in its entirety
        result = await task
        print(result)
    
asyncio.run(main())
