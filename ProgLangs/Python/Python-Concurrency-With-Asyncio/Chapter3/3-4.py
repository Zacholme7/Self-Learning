# 3.4: creating a nonblocking socket
import socket

# create the socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM) 

# set the socket options
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server_address = ('127.0.0.1', 8000) # set the address of the socket
server_socket.bind(server_address) # bind the socket to the address
server_socket.listen() # listen for incomming requests
server_socket.setblocking(False) # make it so that the socket is non blocking
