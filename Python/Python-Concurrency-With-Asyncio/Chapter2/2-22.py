import asyncio
from util import delay

def call_later():
    print("im being called in the future")

async def main():
    # get the event loop
    loop = asyncio.get_running_loop() 

    # run call_later on the next iteration of the event loop
    loop.call_soon(call_later)
    await delay(1)

asyncio.run(main())
