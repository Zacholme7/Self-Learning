# 8.12: Creating an echo server with server objects
import asyncio
import logging
from asyncio import StreamReader, StreamWriter

class ServerState:
    def __init__(self):
        self._writers = [] # the writers for the server

    # add client to the server state and create an echo task
    async def add_clients(self, reader: StreamReader, writer: StreamWriter):
        self._writers.append(writer) # add the writer to our state
        await self._on_connect(writer) # on connection
        asyncio.create_task(self._echo(reader, writer)) # create an echo task

    # on a new connection, tell the client how many users are online and notify others of a new user
    async def _on_connection(self, writer: StreamWriter):
        writer.write(f"welcome {len(self._writers)} user(s) are online\n".encode())
        await writer.drain() # send everything in the write buffer
        await self._notify_all("new user connection") # notify all of the connections

    # send a mesasge to all other users
    async def _notify_all(self, message: str):
        for writer in self._writers:
            try:
                writer.write(message.encode())
                await writer.drain()
            except ConnectionError as e:
                logging.exception("Could not write to client", exc_info = e)
                self._writers.remove(writer)

async def main():
    server_state = ServerState()

    async def client_connected(reader: StreamReader, writer: StreamWriter) -> None:
        # add a client to the server
        # add client will append a writer and print out conneciton information
        await server_state.add_client(reader, writer)

    # start the server
    # client_connected for our callback
    server = await asyncio.start_server(client_connected, "127.0.0.1", 8000)

    # context manager for the server
    async with server:
        await server.serve_forever()

asyncio.run(main())




