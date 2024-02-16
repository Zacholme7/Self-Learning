import functools
import time
from typing import Callable, Any

# decorator to time async funtions
# Decorator to time async functions
def async_timed(func: Callable) -> Callable:
    @functools.wraps(func)
    async def wrapper(*args, **kwargs) -> Any:
        print(f'starting {func.__name__} with args {args}, kwargs {kwargs}')
        start = time.time()
        try:
            return await func(*args, **kwargs)
        finally:
            end = time.time()
            total = end - start
            print(f'finished {func.__name__} in {total:.4f} second(s)')
    return wrapper






