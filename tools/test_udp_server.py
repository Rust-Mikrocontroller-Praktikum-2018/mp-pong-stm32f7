#!/usr/bin/env python
import socket
import sys
from struct import *

UDP_ADDRESS = 'localhost'
UDP_ADDRESS = '141.52.46.1'
UDP_PORT = 2018

MESSAGE = b'\x00\x00\x60d\x01\x90\x00d\x00\xc8\x00d\x00\x01\x00\x01\xff\x00'
data = unpack('>hhhhhhhhBB', MESSAGE)
print(data)

gamestate = (0, 0, 400, 100, 200, 100, 1, 1, 0, 0)
data = pack('>hhhhhhhhBB', gamestate)
print(data)

exit()

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
server_address = (UDP_ADDRESS, UDP_PORT)
sock.bind(server_address)

while True:
    data, address = sock.recvfrom(4096)

    print('received {} ({} bytes) from {}'.format(data, len(data), address))

    
#    if data:
#        sent = sock.sendto(data, address)

