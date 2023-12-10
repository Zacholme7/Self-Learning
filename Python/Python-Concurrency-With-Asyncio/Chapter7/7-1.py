# 7.1: A multithreading echo server
from threading import Thread
import socket

def echo(client: socket):
    while True:
        data = client.recv(2048) # get the data from the client
        print(f"Received {data}, sending")
        client.sendall(data) # send the data back

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server:
    # configure and start server
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", 8000))
    server.listen()
    while True:
        connetion, _ = server.accept() # block waiting for a client to connect
        thread = Thread(target = echo, args = (connection,)) # create new thread to run new connection
        thread.start()



