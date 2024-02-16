# 8.3: A http request with stream readers and writers
import asyncio
from asyncio import StreamReader
from typing import AsyncGenerator

async def read_until_empty(stream_reader: StreamReader) -> AsyncGenerator[str, None]:
    # read a line and decode it until we dont have any left
    while response := await stream_reader.readline():
        yield response.decode()

async def main():
    host: str = "www.example.com"
    request: str = f"GET / HTTP/1.1\r\n" \
                   f"Connection: close\r\n" \
                   f"Host: {host}\r\n\r\n"

    # call open connection to get a reader and a writer
    stream_reader, stream_writer = await asyncio.open_connection("www.example.com", 80)

    try:
        # write to the stream
        stream_writer.write(request.encode())
        # drain will block until all queues data gets sent to the socket
        await stream_writer.drain()

        # read all the responses
        responses = [response async for response in read_until_empty(stream_reader)]

        print("".join(responses))
    finally:
        # close the writer and wait for it to finish closing
        stream_writer.close()
        await stream_writer.wait_closed()

asyncio.run(main())

