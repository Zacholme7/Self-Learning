#4.1: An asynchronous context manager to wait for a client connection
import asyncio
import socket
from types import TracebackType
from typing import Optional, Type

class ConnectedSocket:

    def __init__(self, server_socket):
        self._connection = None
        self._server_socket = server_socket

    # called when we enter the with block
    # waits until a client connects and returns the connection
    async def __aenter__(self):
        print("Entering context manager, waiting for connection")
        # get the event loop
        loop = asyncio.get_event_loop() 

        # accept client connection
        connection, address = await loop.sock_accept(self._server_socket)
        self._connection = connection

        print("Accepted a connection")
        return self._connection

    # called when we exit the with block
    # cleans up any resources
    async def __aexit__(self,
            exc_type: Optional[Type[BaseException]],
            exc_val: Optional[BaseException],
            exc_tb: Optional[TracebackType]):
        print("Exiting context manager")
        self._connection.close()
        print("closed connection")

# main function
async def main():
    loop = asyncio.get_event_loop()

    # set socket options
    server_socket = socket.socket()
    server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server_address = ("127.0.0.1", 8000)
    server_socket.setblocking(False)
    server_socket.bind(server_address)
    server_socket.listen()

    # this calls __aenter__ and waits for a client connection
    async with ConnectedSocket(server_socket) as connection:
        data = await loop.sock_recv(connection, 1024)
        print(data)
        # now __aenter__ will exit and the connection will be closed

asyncio.run(main())
