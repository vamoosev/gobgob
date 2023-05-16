from scapy.all import *

# sniff packets and put to pcap file
def sniff_packets():
    pkts = sniff(iface="wlan0mon")
    print(pkts)
    wrpcap("sniffed.pcap", pkts)



def main():
    sniff_packets()

if __name__ == "__main__":
    main()
