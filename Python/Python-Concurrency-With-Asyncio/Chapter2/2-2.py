import asyncio

async def coroutine_add_one(number):
    return number + 1

def add_one(number):
    return number + 1

function_result = add_one(1)
coroutine_result = coroutine_add_one(1)

print(f"Function result is {function_result} and the type is {type(function_result)}")
print(f"Function result is {coroutine_result} and the type is {type(coroutine_result)}")

# demonstrates that we are getting a coroutine object

#Function result is 2 and the type is <class 'int'>
#Function result is <coroutine object coroutine_add_one at 0x7fdc25ecd340> and the type is <class 'coroutine'>
#sys:1: RuntimeWarning: coroutine 'coroutine_add_one' was never awaited