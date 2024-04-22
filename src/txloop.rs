use crate::tx_setup;
use packet::{builder::Builder, ip};
use std::io::Write;
use std::net::Ipv4Addr;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn tx_loop(mut writer: Writer) {
    let mut device = tx_setup(108, *b"abcde", 17, 0, 0);

    let message = b"sendtest";

    loop {
        let message_packet = ip::v4::Builder::default()
            .id(0x42)
            .unwrap()
            .ttl(64)
            .unwrap()
            .source(Ipv4Addr::new(192, 168, 12, 100))
            .unwrap()
            .destination(Ipv4Addr::new(192, 168, 12, 102))
            .unwrap()
            .icmp()
            .unwrap()
            .echo()
            .unwrap()
            .request()
            .unwrap()
            .identifier(0)
            .unwrap()
            .sequence(0)
            .unwrap()
            .payload(message)
            .unwrap()
            .build();

        match message_packet {
            Ok(message_packet_bytes) => {
                writer.write_all(&message_packet_bytes).unwrap();

                for byte in message_packet_bytes.iter() {
                    match device.push(0, &[*byte]) {
                        Ok(()) => {
                            // Push successful
                        }
                        Err(err) => {
                            println!("Error pushing to device: {:?}", err);
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error building reply packet: {:?}", err);
            }
        }

        match device.send() {
            Ok(retries) => {
                println!("Message sent, {} retries needed", retries);

                if device.data_available().unwrap() {
                    device
                        .read_all(|packet| {
                            println!("Received back {:?} bytes", packet.len());
                            println!("ACK Payload {}", String::from_utf8_lossy(packet));
                        })
                        .unwrap();
                } else {
                    println!("Blank ACK");
                }
            }
            Err(err) => {
                println!("Destination unreachable: {:?}", err);
                device.flush_output().unwrap();
            }
        }
        sleep(Duration::from_millis(5000));
    }
}
