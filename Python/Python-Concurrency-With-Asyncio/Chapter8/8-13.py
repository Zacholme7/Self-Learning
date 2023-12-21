# 8.13: A chat server
import asyncio
import logging
from asyncio import StreamReader, StreamWriter

class ChatServer:
    def __init__(self):
        # maps the client usernames to the writers
        self._username_to_writer = {}

    # create the server
    async def start_chat_server(self, host: str, port: int):
        server = await asyncio.start_server(self.client_connected, host, port)

        async with server:
            await server.serve_forever()

    # function to call when a new client connects, our callback
    async def client_connected(self, reader: StreamReader, writer: StreamWriter):
        command = await reader.readline() # get the username
        print(f"CONNECTED {reader} {writer}")
        command, args = command.split(b' ')

        if command = b'CONNECT':
            username = args.replace(b'\n', b'').decode() # get rid of the endline
            self._add_user(username, reader, writer) # add a new user and create a listener for the user
            await self._on_connect(username, writer) # on connect function
        else:
            logging.error("Got invalid command from client, disconnecting")
            writer.close()
            await writer.wait_closed()

    def _add_user(self, username: str, reader: StreamReader, writer: StreamWriter):
        self._username_to_writer[username] = writer # add the new user
        asyncio.create_task(self._listen_for_messages(username, reader)) # create a listener for the user

    async def _on_connect(self, username: str, writer: StreamWriter, reader: StreamReader):
        # write to the writer
        writer.write("Welcome {len(self._username_to_writer)} user(s) are online\n".encode())
        await writer.drain() # clear the buffer
        await self._notify_all(f"{username} connected\n") # notify the rest of the clients

    async def _remove_user(self, username: str):
        # get the writer and delete it from our mapping
        writer = self._username_to_writer[username]
        del self._username_to_writer[username]

        try:
            writer.close()
            await writer.wait_closed()
        except Exception as e:
            logging.exception("Error closing client writer, ignoring", exc_info = e)

    # listen for messages from a client and send them to all other clients
    async def _listen_for_messages(self, username: str, reader: StreamReader):
        try:
            while (data := await asyncio.wait_for(reader.readline(), 60)) != b'':
                await self._notify_all(f"{username}: {data.decode()}")
            await self._notify_all(f"{username} has left the chat\n")
        except Exception as e:
            logging.exception("Error reading from client", exc_info = e)
            await self._remove_user(username)

    # send a message to all connected clients, remove any disconnected ones
    async def _notify_all(self, message: str):
        inactive_users = {}
        for username, writer in self._username_to_writer.items():
            try:
                writer.write(message.encode())
                await writer.drain()
            except ConnectionError as e:
                logging.exception("Could not write to client", exc_info = e)
                inactive_users.append(username)

        [await self._remove_user(username) for username in inactive_users]

async def main():
    chat_server = ChatServer()
    await chat_server.start_chat_server("127.0.0.1", 8000)

asyncio.run(main())





