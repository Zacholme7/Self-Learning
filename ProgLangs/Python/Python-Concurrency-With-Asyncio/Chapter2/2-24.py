import asyncio

async def main():
    loop = asyncio.get_event_loop()
    loop.slow_callback_duration = .250 # set the debug duration

asyncio.run(main(), debug = True)

