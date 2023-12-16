# 8.1: Running a HTTP request with transports and protocols
import asyncio
from asyncio import Transport, Future, AbstractEventLoop
from typing import Optional

# define a class that inherits from the protocol base class
class HTTPGetClientProtocol(asyncio.Protocol):
    def __init__(self, host: str, loop: AbstractEventLoop):
        self._host: str = host
        self._future: Future = loop.create_future() # this will hold the data we have receieved
        self._transport: Optional[Transport] = None
        self._reponse_buffer: bytes = b''

    # await the internal future until we get a response from the server
    async def get_response(self):
        return await self._future

    # create the http request
    def _get_request_bytes(self) -> bytes:
        request = f"GET / HTTP/1.1\r\n" \
                  f"Connection: close\r\n" \
                  f"Host: {self._host}\r\n\r\n"
        return request.encode()

    # function to make the connection
    def connection_made(self, transport: Transport):
        print(f"Connection made to {self._host}")
        self._transport = transport
        # once we have established a conneciton, use transport to send the request
        self._transport.write(self._get_request_bytes())

    # function to recieve data
    def data_received(self, data):
        print(f"Data received")
        # save the data to our internal buffer
        self._response_buffer = self._response_buffer + data

    # handle when we receieve a eof
    def eof_received(self) -> Optional[bool]:
        # set the result of the future
        self._future.set_result(self._response_buffer.decode())
        return False

    def connection_lost(self, exc: Optional[Exception]) -> None:
        if exc is None:
            # if connection closes without error, do nothing
            print("Connection closed without error")
        else:
            # if there is an error, complete the future with the exception
            self._future.set_exception(exc)


