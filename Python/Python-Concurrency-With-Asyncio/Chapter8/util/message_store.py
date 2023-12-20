from collections import deque
from typing import Callable, Deque, Awaitable

class MessageStore:
    def __init__(self, callback: Callback[[Deque], Awaitable[None]], max_size: int):
        self._deque = deque(maxlen = max_size) # make our deque
        self._callback = callback # the callback function

    async def append(self, item):
        self._deque.append(item)
        await self._callback(self._deque)
