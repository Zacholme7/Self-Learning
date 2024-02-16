# 11.2: A single threaded race condition
import asyncio

counter: int = 0

async def increment():
    global counter
    # simulate the way machine instructrs are ran for a increment
    temp_counter = counter
    temp_counter = temp_counter + 1
    await asyncio.sleep(0.01)
    counter = temp_counter

async def main():
    global counter
    for _ in range(1000):
        tasks = [asyncio.create_task(increment()) for _ in range(100)]
        print(f"Counter is {counter}")
        assert counter == 100
        counter = 0

asyncio.run(main())

