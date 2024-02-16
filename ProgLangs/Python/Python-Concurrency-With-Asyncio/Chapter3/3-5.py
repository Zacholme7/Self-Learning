# 3.5: A first attempt at a non blocking server
# this is the same as 3-3 except we set it as non blocking
import socket

# create the socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM) 

# set the socket options
server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server_address = ('127.0.0.1', 8000) # set the address of the socket
server_socket.bind(server_address) # bind the socket to the address
server_socket.listen() # listen for incomming requests
server_socket.setblocking(False) # mark the server socket as nonblocking


connections = [] # list to hold all of the collections

try:
    while True:
        connection, client_address = server_socket.accept() # accept a client connection
        connection.setblocking(False) # mark the client socket as nonblocking
        print(f"I got a connection from {client_address}")
        connection.append(connection)

        for connection in connections:
            buffer = b'' # this is the buffer that we will hold incomming info in

            # while we have not receieved a carraige return
            while buffer[-2:] != b'\r\n':
                data = connection.recv(2) # recieve two bytes
                if not data:
                    break
                else:
                    print(f"I got data: {data}")
                    buffer = buffer + data # append the data to our buffer

            print(f"All the data is: {buffer}")
            connection.sendall(buffer)
finally:
    server_socket.close()
        
