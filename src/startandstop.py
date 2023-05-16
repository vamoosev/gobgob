import os
import sys

# run a command if argument is 'start' or 'stop'
if sys.argv[1] == 'start':
    os.system('killall NetworkManager')
    os.system("killall wpa_supplicant")
    os.system('airmon-ng start wlan0')

elif sys.argv[1] == 'stop':
    os.system('airmon-ng stop wlan0mon')
    os.system('systemctl start NetworkManager')
    os.system('systemctl start wpa_supplicant')

