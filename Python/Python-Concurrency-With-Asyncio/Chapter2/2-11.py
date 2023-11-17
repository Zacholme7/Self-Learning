import asyncio
from asyncio import CancelledError
from util import delay

async def main():
    # will run for 10 seconds and we want to cancel it after 10
    long_task = asyncio.create_task(delay(10))

    seconds_elapsed = 0

    # keep checking if it has finished
    while not long_task.done():
        print("Task not finished, checking again in a second")
        # sleep for a second and then cancel it when we are at 5 seconds
        await asyncio.sleep(1)
        seconds_elapsed = seconds_elapsed + 1
        if seconds_elapsed == 5:
            long_task.cancel()

    # when we await it, it will raise exception since we have cancelled it
    try:
        await long_task
    except:
        print("Our task was cancelled")

asyncio.run(main())
