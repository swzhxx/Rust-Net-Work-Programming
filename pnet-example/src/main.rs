use std::env;
extern crate pnet;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("获得interface失败");
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("An Error occurred when create the datalink chennel {}", e)
        }
    };

    // Loop over packets arriving on the given interface
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            }
            Err(e) => {
                panic!("An error occurred while reading {}", e);
            }
        }
    }
}

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!(
                                "Got a Tcp packet {} : {} to {} :{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            )
                        }
                    }
                    _ => println!("ignoring"),
                }
            }
        }
        _ => {
            println!("ignoring");
        }
    }
}
