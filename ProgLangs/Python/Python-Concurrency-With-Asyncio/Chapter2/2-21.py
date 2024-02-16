import asyncio

async def main():
    await asyncio.sleep(1)

# create a new event loop
loop = asyncio.new_event_loop()

try:
    # run the function until it is done
    loop.run_until_complete(main())
finally:
    # clean up the event loop
    loop.close()
