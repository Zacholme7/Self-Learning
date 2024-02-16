from asyncio import Future
import asyncio

def make_request() -> Future:
    future = Future()
    asyncio.create_task(set_future_value(future)) # create a task to async set val of future
    return future

async def set_future_value(future) -> None:
    await asyncio.sleep(1) # wait for one value before setting value of future
    future.set_result(42)

async def main():
    future = make_request() # construct the future
    print(f"Is the future done? {future.done()}")
    value = await future # await the future, this will execute it and pause main
    print(f"Is the future done? {future.done()}")
    print(value)

asyncio.run(main())


