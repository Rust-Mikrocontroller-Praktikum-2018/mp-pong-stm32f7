#!/usr/bin/env python
import socket


# UDP_IP ='141.52.46.1'
UDP_IP = '141.52.46.2'
#UDP_IP = 'localhost'
UDP_PORT = 2018
# MESSAGE = b'\x01' # down

MESSAGE = b'\x00\x00\x60d\x01\x90\x00d\x00\xc8\x00d\x00\x01\x00\x01\xff\x00' # gamestate


sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(MESSAGE, (UDP_IP, UDP_PORT))
#data,server = sock.recvfrom(4096)
#print('received {!r}'.format(data))

