from asyncio import Future

# construct the future
my_future = Future()

print(f"is my_future done? {my_future.done()}")

# set the result, this will make it done
my_future.set_result(42);

print(f"is my_future done? {my_future.done()}")
print(f"what is the result of my future? {my_future.result()}")
