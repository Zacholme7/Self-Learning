import asyncio
from util import delay

async def add_one(number: int) -> int:
    return number + 1

async def hello_world_message() -> str:
    await delay(1)
    return "hello world"

async def main() -> None:
    message = await hello_world_message() # pause main till this returns
    one_plus_one = await add_one(1) # pause main until this returns
    print(one_plus_one)
    print(message)

# kick off the async application
asyncio.run(main())

# why is this not running concurrently?
# await pauses our current coroutine and wont execute any other code inside the coroutine until it gives us a value
