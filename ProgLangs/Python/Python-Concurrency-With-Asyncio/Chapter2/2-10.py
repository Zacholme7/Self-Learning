import asyncio
from util import delay

# this 
async def hello_every_second():
    for i in range(2):
        await asyncio.sleep(1)
        print("Im running other code while im waiting")

async def main():
    first_delay = asyncio.create_task(delay(3))
    second_delay = asyncio.create_task(delay(3))

    # will run all of the tasks
    # will run the delays and then give up execution while i/o
    # and run hello function
    await hello_every_second()
    await first_delay
    await second_delay

asyncio.run(main())

# output
#sleeping for 3 second(s)
#sleeping for 3 second(s)
#Im running other code while im waiting
#Im running other code while im waiting
#finished sleeping for 3 second(s)
#finished sleeping for 3 second(s)
