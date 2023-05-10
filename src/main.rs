use pcap::{Capture, Device};

fn main() {
    // Find the WiFi interface
    let device = Device::lookup()
        .unwrap()
        .into_iter()
        .find(|dev| dev.name == "wlan0")
        .unwrap();

    // Open the interface in promiscuous mode
    let mut cap = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .open()
        .unwrap();

    // Capture packets and print their contents
    while let Ok(packet) = cap.next() {
        println!("{:?}", packet);
    }
}
