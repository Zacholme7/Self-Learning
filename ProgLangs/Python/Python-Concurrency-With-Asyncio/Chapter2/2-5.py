import asyncio


async def hello_world_message() -> str:
    await asyncio.sleep(1) # pause for one second to simulate the message, can be running other functions while this sleeps

async def main() -> None:
    message = await hello_world_message() # pause main until the coroutine finishes
    print(message)

asyncio.run(main())