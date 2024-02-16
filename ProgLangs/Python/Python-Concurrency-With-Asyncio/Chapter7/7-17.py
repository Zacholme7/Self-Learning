# 7.17: Hashing wtih multithreading and asycio
import asyncio
import functools
import hashlib
import os
from concurrent.futures.thread import ThreadPoolExecutor
import random
import string
from util import async_timed

# function to generate a random password
def random_password(length: int) -> bytes:
    ascii_lowercase = string.ascii_lowercase.encode()
    return b''.join(bytes(random.choice(ascii_lowercase)) for _ in range(length))

# generate random passwords of length 10
passwords = [random_password(10) for _ in range(10000)]

# function to hash the password
def hash(password: bytes) -> str:
    salt = os.urandom(16) # our salt for the hash
    return str(hashlib.scrypt(password, salt = salt, n = 2048, p = 1, r = 8))

@async_timed
async def main():
    loop = asyncio.get_running_loop()
    tasks = []

    with ThreadPoolExecutor() as pool:
        for password in passwords:
            tasks.append(loop.run_in_executor(pool, functools.partial(hash, password)))
    await asyncio.gather(*tasks)

asyncio.run(main())


