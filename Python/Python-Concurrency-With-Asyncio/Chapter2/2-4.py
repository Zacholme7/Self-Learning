import asyncio

async def add_one(number):
    return number + 1

async def main():
    one_plus_one = await add_one(1) # this will pause and wait for function to finish
    two_plus_one = await add_one(2) # this will pause and wait for function to finish
    print(one_plus_one)
    print(two_plus_one)

# entry to the main application
asyncio.run(main())