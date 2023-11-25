# 3.7: Using selectors to build a non blocking server
import selectors
import socket
from selectors import SelectorKey
from typing import List, Tuple

# create a selector
selector = selectors.DefaultSelector()

# create the socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM) 

# set the socket options
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server_address = ('127.0.0.1', 8000) # set the address of the socket
server_socket.bind(server_address) # bind the socket to the address
server_socket.listen() # listen for incomming requests
server_socket.setblocking(False) # set the server socket as nonblocking

# register the socket with a read event
selector.register(server_socket, selectors.EVENT_READ)


while True:
    # create a selector that will timeout after 1 second
    events: List[Tuple[SelectorKey, int]] = selector.select(timeout = 1) 

    # if there are no events, print it out
    # this happends when a timeout occurs
    if len(events) == 0:
        print("no events, waiting a bit more")

    for event, _ in events:
        event_socket = event.fileobj

        # if the event socket is the same as the server socket
        # then we know this is a connection attempt
        if event_socket == server_socket:
            connection, address = server_socket.accept()
            connection.setblocking(False)
            print(f"I got a connection from {address}")
            selector.register(connection, selectors.EVENT_READ)
        else:
            data = event_socket.recv(1024)
            print(f"I got some data: {data}")
            event_socket.send(data)
        
