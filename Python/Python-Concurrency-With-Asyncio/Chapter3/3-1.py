#3.1: Starting a server and listening for a connection
# run the application
# in new terminal run "telnet localhost 8000"

import socket

# create the socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# set the socket options
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server_address = ('127.0.0.1', 8000) # set the address of the socket
server_socket.bind(server_address) # bind the socket to the address
server_socket.listen() # listen for incomming requests

connection, client_address = server_socket.accept() # wait for a connection
print(f"I got a connection from {client_address}")
