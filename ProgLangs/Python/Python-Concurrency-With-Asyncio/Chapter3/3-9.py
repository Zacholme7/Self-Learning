#3.9: Adding a signal handler to cancel all tasks
import asyncio, signal
from asyncio import AbstractEventLoop
from typing import Set

from util import delay

# cancel all of the tasks
def cancel_tasks():
    print("Got SIGNINT")
    tasks: Set[asyncio.Task] = asyncio.all_tasks()
    print(f"Cancelling {len(tasks)} task(s)")
    [task.cancel() for task in tasks]

async def main():
    loop: AbstractEventLoop = asyncio.get_running_loop()

    # add a signal handler for the cancel tasks coroutine
    loop.add_signal_handler(signal.SIGINT, cancel_tasks)
    await delay(10)

asyncio.run(main())

