use crate::tx_setup;
use packet::{builder::Builder, ip};
use std::io::Write;
use std::net::Ipv4Addr;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn tx_loop(mut writer: Writer) {
    let mut device = tx_setup(108, *b"abcde", 17, 0, 0);

    let message = b"sendtest and we need to have a really long message to be sure we can send stuff across several packages so this string is very very very long!!!";

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

    loop {
        let mut buf = [0u8; 4096];

        // get nubmer of bytes in message_packet
        let n = message_packet.as_ref().unwrap().len();

        // put message_packet into buf using slice
        buf[..n].copy_from_slice(message_packet.as_ref().unwrap());

        let pkt = &buf[0..n];

        let mut chunks = vec![];
        let mut queue = vec![];

        // Split the packet into 32 byte chunks that can we push to the device
        for chunk in pkt.chunks(32) {
            queue.push(chunk);
            if queue.len() == 2 {
                chunks.push(queue.clone());
                queue.clear();
            }
        }
        if !queue.is_empty() {
            chunks.push(queue);
        }
        for slice in &chunks {
            for element in slice {
                device.push(0, element).unwrap();
            }
            match device.send() {
                Ok(retries) => {
                    println!("message sent, {} retries needed", retries);
                    if device.data_available().unwrap() {
                        device
                            .read_all(|packet| {
                                println!("Received back {:?} bytes", packet.len());
                                println!("ACK Payload {:?}", packet);
                            })
                            .unwrap();
                    } else {
                        println!("Blank ACK")
                    }
                }
                Err(err) => {
                    println!("destination unreachable: {:?}", err);
                }
            };
            sleep(Duration::from_millis(5000));
        }
        let pkt = &buf[0..n];
        let result = writer.write(pkt).unwrap();
        println!("A packet containing {} bytes written to interface", result);
    }
}
