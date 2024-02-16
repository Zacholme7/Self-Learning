# 8.5: An asynchronous standard input reader
import asyncio
from asyncio import StreamReader
import sys

async def create_stdin_reader() -> StreamReader:
    # stream reader to read input
    stream_reader = asyncio.StreamReader()
    # create a protocol from the stream reader
    protocol = asyncio.StreamReaderProtocol(stream_reader)
    # connect the protocol to stdin
    loop = asyncio.get_running_loop()
    await loop.connect_read_pipe(lambda: protocol, sys.stdin)
    # return the stream reader
    return stream_reader
