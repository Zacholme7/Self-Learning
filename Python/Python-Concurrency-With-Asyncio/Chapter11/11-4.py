# 11.4: Using an asyncio lock
import asyncio
from asyncio import Lock
from util import delay 

async def a(lock: Lock):
    print("coroutine a waiting to acquire the lock")
    async with lock:
        print("courtine a is in the critical section")
        await delay(2)
    print("coroutine b released the lock")

async def b(lock: Lock):
    print("coroutine b waiting to acquire the lock")
    async with lock:
        print("courtine b is in the critical section")
        await delay(2)
    print("coroutine b released the lock")

async def main():
    lock = Lock()
    await asyncio.gather(a(lock), b(lock))

asyncio.run(main())


