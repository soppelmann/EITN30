use crate::tx_setup;
use crate::{BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE, TX_RETRIES};
//use packet::{builder::Builder, ip};
use std::io::Read;
//use std::net::Ipv4Addr;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Reader;

pub fn tx_loop(mut reader: Reader) {
    let mut device = tx_setup(108, *b"abcde", 17, 0, 0);

    // let message = b"sendtest and we need to have a really long message to be sure we can send stuff across several packages so this string is very very very long!!!";

    // let message_packet = ip::v4::Builder::default()
    //     .id(0x42)
    //     .unwrap()
    //     .ttl(64)
    //     .unwrap()
    //     .source(Ipv4Addr::new(192, 168, 12, 100))
    //     .unwrap()
    //     .destination(Ipv4Addr::new(192, 168, 12, 102))
    //     .unwrap()
    //     .icmp()
    //     .unwrap()
    //     .echo()
    //     .unwrap()
    //     .request()
    //     .unwrap()
    //     .identifier(0)
    //     .unwrap()
    //     .sequence(0)
    //     .unwrap()
    //     .payload(message)
    //     .unwrap()
    //     .build();

    loop {
        let mut buf = [0u8; BUFFER_SIZE];

        //TODO: This currently causes error on tx side
        let result = reader.read(&mut buf);
        match result {
            Ok(n) => {
                println!("{} bytes read from interface", n);

                // get nubmer of bytes in message_packet
                //let n = message_packet.as_ref().unwrap().len();

                // put message_packet into buf using slice
                //buf[..n].copy_from_slice(message_packet.as_ref().unwrap());

                let pkt = &buf[0..n];

                // Split the packet into 32 byte chunks that can we push to the device
                for queue in pkt.chunks(PACKET_SIZE * QUEUE_SIZE) {
                    queue.chunks(PACKET_SIZE).for_each(|pkt| {
                        // push the packet to the 3 length FIFO_QUEUE
                        device.push(0, pkt).unwrap();
                    });

                    // attempt transmit 10 times using device.send() with a for loop for erros
                    for i in 0..TX_RETRIES {
                        match device.send() {
                            Ok(retries) => {
                                println!("message sent, {} retries needed", retries);
                                break;
                            }
                            Err(err) => {
                                println!("destination unreachable: {:?}", err);
                                if i == 9 {
                                    println!("max retries reached: {}, flushing output", i);
                                    device.flush_output().unwrap();
                                }
                                println!("reached: {}", i);
                            }
                        };
                    }

                    sleep(Duration::from_millis(50));
                }
            }
            Err(err) => {
                println!("{} error when reading from interface", err)
            }
        }
    }
}
