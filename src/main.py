from scapy.all import *

def packet_handler(pkt):
    if pkt.haslayer(Dot11):
        if pkt.type == 0 and pkt.subtype == 4:
            print("Probe Request:", pkt.addr2)

sniff(iface="wlan0", prn=packet_handler)
