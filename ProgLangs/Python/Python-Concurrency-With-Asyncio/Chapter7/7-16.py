# 7.16: Hashing passwords with scrypt
import hashlib
import os
import string
import time
import random

# generate a random password
def random_password(length: int) -> bytes:
    ascii_lowercase = string.ascii_lowercase.encode()
    return b''.join(bytes(random.choice(ascii_lowercase)) for _ in range(length))

# create a bunch of random passwords
passwords = [random_password(10) for _ in range(10000)]

# hash the password
def hash(password: bytes) -> str:
    salt = os.urandom(16)
    return str(hashlib.scrypt(password, salt = salt, n = 2048, p = 1, r = 8))

start = time.time()

# hash all the passwords
for password in passwords:
    hash(password)

end = time.time()
print(end - start)



