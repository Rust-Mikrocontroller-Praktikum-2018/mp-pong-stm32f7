#!/usr/bin/env python
import socket


UDP_IP ='141.52.46.198'
#UDP_IP = 'localhost'
UDP_PORT = 2018
MESSAGE = b'hey'
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(MESSAGE, (UDP_IP, UDP_PORT))
#data,server = sock.recvfrom(4096)
#print('received {!r}'.format(data))

