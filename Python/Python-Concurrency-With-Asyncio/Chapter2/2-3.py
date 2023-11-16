import asyncio

async def coroutine_add_one(number):
    return number + 1

# call the function to make the co routine object
# call the run function that will execute the coroutine object
result = asyncio.run(coroutine_add_one(1))

# what is asyncio.run doing?
# creates a brand new event
# takes the coroutine in it and runs it until it completes and returns the result
# also does some cleanup and shutsdown/closes event loop
# intended to be the main entry point into the asyncio application that we create
# only executes one coroutine and that coroutine should launch all other aspects of application