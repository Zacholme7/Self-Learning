#3.8: Building an asyncio echo server
import asyncio
import socket
from asyncio import AbstractEventLoop

async def echo(connection: socket, loop: AbstractEventLoop) -> None:
    # loop forever waiting for data from a client connection
    while data := await loop.sock_recv(connection, 1024):
        # if we get data, send it out to all connections
        await loop.sock_sendall(connection, data)

async def listen_for_connection(server_socket: socket, loop: AbstractEventLoop):
    while True:
        # get a connection
        connection, address = await loop.sock_accept(server_socket)
        # set it to non blocking
        connection.setblocking(False)

        print(f"I got a connection from {address}")

        # create an echo task to listen for client data
        asyncio.create_task(echo(connection, loop))

async def main():
    # create the server socket
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # set the socket options
    server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

    # configure the socket
    server_address = ('127.0.0.1', 8000)
    server_socket.setblocking(False)
    server_socket.bind(server_address)
    server_socket.listen()

    await listen_for_connection(server_socket, asyncio.get_event_loop())


asyncio.run(main())





