from scapy.all import *
import requests
import socket
import datetime
import time
import json
import pickle


def main():
    r = requests.post(
        f"http://localhost:8080/supersalainen/{time.localtime().tm_mday}:{time.localtime().tm_mon}_{socket.gethostname()}",
        data=json.dumps(sniff_packets()),
    )


# def pkt2dict(pkt):
#     packet_dict = {}
#     for line in pkt.show2(dump=True).split('\n'):
#         if '###' in line:
#             if '|###' in line:
#                 sublayer = line.strip('|#[] ')
#                 packet_dict[layer][sublayer] = {}
#             else:
#                 layer = line.strip('#[] ')
#                 packet_dict[layer] = {}
#         elif '=' in line:
#             if '|' in line and 'sublayer' in locals():
#                 key, val = line.strip('| ').split('=', 1)
#                 packet_dict[layer][sublayer][key.strip()] = val.strip('\' ')
#             else:
#                 key, val = line.split('=', 1)
#                 val = val.strip('\' ')
#                 if(val):
#                     try:
#                         packet_dict[layer][key.strip()] = eval(val)
#                     except:
#                         packet_dict[layer][key.strip()] = val
#         else:
#             pass
#     return packet_dict


# sniff packets and put to pcap file
def sniff_packets():
    pkts = sniff(iface="wlan0mon", count=100)
    macaddrs = []
    for pkt in pkts:
        if pkt.haslayer(Dot11):
            if pkt.type == 0 and pkt.subtype == 4:
                macaddrs.append(pkt.addr2)
    # remove duplicates
    macaddrs = [*set(macaddrs)]
    return macaddrs


if __name__ == "__main__":
    main()
