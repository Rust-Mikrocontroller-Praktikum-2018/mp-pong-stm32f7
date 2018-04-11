#!/usr/bin/env python
import socket
import sys

UDP_ADDRESS = 'localhost'
UDP_ADDRESS = '141.52.46.2'
UDP_PORT = 2018

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
server_address = (UDP_ADDRESS, UDP_PORT)
sock.bind(server_address)

while True:
    data, address = sock.recvfrom(4096)
    print('received {} ({} bytes) from {}'.format(data, len(data), address))
#    if data:
#        sent = sock.sendto(data, address)

