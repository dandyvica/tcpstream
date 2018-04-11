#!/usr/local/python3

import socket
import time


TCP_IP = '127.0.0.1'
TCP_PORT = 8081
BUFFER_SIZE = 1024
MESSAGE = b"Hello, World!"

#
NB_SOCKS = 5

# create 5 sockets cnx
cnx = []
for i in range(0,NB_SOCKS):
    cnx.append(socket.socket(socket.AF_INET, socket.SOCK_STREAM))
    cnx[i].connect((TCP_IP, TCP_PORT))

# now send data to each socket
for s in cnx:
    nb_sent = s.send(MESSAGE)
    print("{} data sent!".format(nb_sent))   

time.sleep(10) 

# close all sockets
for s in cnx:
    print("Closing socket")
    s.close()
