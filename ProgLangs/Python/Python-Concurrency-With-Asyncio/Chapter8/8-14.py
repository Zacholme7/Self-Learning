# 8.14: the chat client
import asyncio
import os
import logging
import tty
from asyncio import StreamReader, StreamWriter
from util import *


async def send_message(message: str, writer: StreamWriter):
    writer.write((message + '\n').encode())
    await writer.drain()

async def listen_for_messages(reader: StreamReader, message_store: MessageStore):
    while (message := await reader.readline()) != b'':
        await message_store.append(message.decode())
    await message_store.append("server closed connection")

async def read_and_send(stdin_reader: StreamReader, writer: StreamWriter):
    while True:
        message = await read_line(stdin_reader)
        await send_message(message, writer)

async def main():
    async def redraw_output(items: deque):
        save_cursor_position()
        move_to_top_of_screen()

        for item in items:
            delete_line()
            sys.stdout.write(item)
        restore_cursor_position(f

    ttyp.setcbreak(0)
    os.system("clear")
    rows = move_to_bottom_of_screen()

    messages = MessageStore(redraw_output, rows - 1)

    stdin_reader = await create_stdin_reader()
    sys.stdout.write("enter username: ")
    username = await read_line(stdin_reader)

    reader, writer = await asyncio.open_connection("127.0.0.1", 8000)

    writer.write(f"Connect {username\n".encode())
    await writer.drain()

    message_listener _ asyncio.create_task(listen_for_messages(reader, messages))
    input_listener = asyncio.create_task(read_and_send(stdin_reader, writer))


    try:
        await asyncio.wait([message_listener, input_listener], return_when= asyncio.FIRST_COMPLETED)
    except Exception as e:
        logging.exception(e)
        writer.close()
        await writer.wait_closed()

asyncio.run(main())
    



