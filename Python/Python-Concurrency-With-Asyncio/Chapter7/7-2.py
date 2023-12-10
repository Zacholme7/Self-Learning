# 7.2: Sublcassing the thread class for a clean shutdown
from threading import Thread
import socket

class ClientEchoThread(Thread):
    # init the thread class
    def __init__(self, client):
        super().__init__()
        self.client = client

    # override the run method
    def run(self):
        try:
            # keep receiving data while we can
            while True:
                data = self.client.recv(2048) # get the data
                # raise exception if there is no data
                if not data:
                    raise BrokenPipeError("Connection closed")
                print(f"Received {data}, sending")
                self.client.sendall(data)
        except OSError as e:
            print(f"Thread interrupted by {e} exception, shutting down")

    # upon thread close
    def close(self):
        # if it is still alive, send message that we are shutting down
        # a thread is avlid if the run method is executing
        if self.is_alive():
            self.client.sendall(bytes("Shutting down", encoding = "utf-8"))
            self.client.shutdown(socket.SHUT_RDWR)


# create our socket
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server:
    # set the options
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", 8000))
    server.listen()

    # all our connected threads
    connection_threads = []
    try:
        while True:
            # accept a socket
            connection, addr = server.accept()
            # create a new thread and start it
            thread = ClientEchoThread(connection)
            connection_threads.append(thread)
            thread.start()
    except:
        # upon exception, close all the threads
        print("shutting down")
        [thread.close() for thread in connection_threads]

