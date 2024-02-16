# 11.10: Event basics
import asyncio
import functools
from asyncio import Event

def trigger_event(event: Event):
    event.set() # will set the internal flag to true
    
async def do_work_on_event(event: Event):
    print("Waiting for event...")
    await event.wait() # wait until the event occurs
    print("Performing work")
    await asyncio.sleep(1) # once event occurs, will no longer block and we can do work
    print("Finished work")
    event.clear() # reset the event so future calls to wait will block

async def main():
    event = asyncio.Event()
    # trigger the event 5 seconds into the future
    asyncio.get_running_loop().call_later(5.0, functools.partial(trigger_event, event))
    await asyncio.gather(do_work_on_event(event), do_work_on_event(event))

asyncio.run(main())
