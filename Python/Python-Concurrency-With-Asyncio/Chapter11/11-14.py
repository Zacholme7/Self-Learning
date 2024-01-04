# 11.14: Condition basics
import asyncio
from asyncio import Condition

async def do_work(condition: Condition):
    while True:
        print("Waiting for condition lock")
        async with condition: # async with syntax to lock condition
            print("Acquired lock, releasinga nd waiting for condition")
            await condition.wait() # wait for event, reacquire lock when it does
            print("Condition was fired, reaquiring lock and doing work")
            await asyncio.sleep(1)
        print("Work finished lock, released")

async def fire_event(condition: Condition):
    while True:
        await asyncio.sleep(5)
        print("About to notify, acquiring condition lock")
        async with condition:
            print("lock acquired notifying all workers")
            condition.notify_all()
        print("notification finished, releasing lock")

async def main():
    condition = Condition()
    asyncio.create_task(fire_event(condition))
    await asyncio.gather(do_work(condition), do_work(condition))

asyncio.run(main())

