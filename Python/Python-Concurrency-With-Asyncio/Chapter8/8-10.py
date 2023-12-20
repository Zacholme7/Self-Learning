# 8.10: The asynchronous delay application
import asyncio
import os
import tty
from collections import deque
from util import *

async def sleep(delay: int, message_store: MessageStore):
    await message_store.append(f"Starting delay {delay}")
    await asycio.sleep(delay)
    await message_store.append(f"Finished delay {delay}")

async def main():
    tty.setcbreak(sys.stdin)
    os.system("clear")
    rows = move_to_bottom_of_screen()

    async def redraw_output(items: deque):
        save_cursor_position()
        move_to_top_of_screen()
        for item in items:
            delete_line()
            print(item)
        restore_cursor_position()

    message = MessageStore(redraw_output, rows - 1)
    stdin_reader = await create_stdin_reader()

    while True:
        line = await read_line(stdin_reader)
        delay_time = int(line)
        asyncio.create_task(sleep(delay_time, message))

asyncio.run(main())
