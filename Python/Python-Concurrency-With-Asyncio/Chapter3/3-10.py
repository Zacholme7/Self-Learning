# 3.10: A graceful shutdown
import asyncio
from asyncio import AbstractEventLoop
import socket
import logging
import signal
from typing import List

async def echo(connection: socket, loop: AbstractEventLoop) -> None:
    try:
        # continuously get data
        while data := await loop.sock_recv(connection, 1024):
            print("got data")
            # if we got boom, raise an exception
            if data == b'boom\r\n':
                raise Exception("unexpected network error")
            await loop.sock_sendall(connection, data)
    except Exception as ex:
        logging.exception(ex)
    finally:
        connection.close()

echo_tasks = []

async def connection_listener(server_socket, loop):
    # recieve and register all incomming connections
    while True:
        connection, address = await loop.sock_accept(server_socket)
        connection.setblocking(false)
        print(f"got a connection from {address}")
        echo_task = asyncio.create_task(echo(connection, loop))
        echo_tasks.append(echo_task)

class GracefulExit(SystemExit):
    pass


async def close_echo_tasks(echo_tasks: List[asyncio.Task]): 
    waiters = [asyncio.wait_for(task, 2) for task in echo_tasks] 
    for task in waiters:
        try:
            await task
        except asyncio.exception.TimeoutError:
            # we expect a timeout here
            pass

async def main():
    # setup the socket configuration
    server_socket = socket.socket() server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server_address = ('127.0.0.1', 8000)
    server_socket.setblocking(False)
    server_socket.bind(server_address)
    server_socket.listen()

    # add the signal handler
    for signame in {'SIGINT', 'SIGTERM'}: 
        loop.add_signal_handler(getattr(signal, signame), shutdown)
    await connection_listener(server_socket, loop)

# get the event loop
loop = asyncio.new_event_loop()

try:
    loop.run_until_complete(main())
except GracefulExit: 
    loop.run_until_complete(close_echo_tasks(echo_tasks))
finally:
    loop.close()

