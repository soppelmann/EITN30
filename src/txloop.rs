use crate::{BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE, TX_RETRIES};
//use packet::{builder::Builder, ip};
use std::io::Read;
//use std::net::Ipv4Addr;
use nrf24l01::NRF24L01;
//use std::thread::sleep;
//use std::time::Duration;
use tun2::platform::posix::Reader;

pub fn tx_loop(mut device: NRF24L01, mut reader: Reader) {
    loop {
        let mut buf = [0u8; BUFFER_SIZE];
        let result = reader.read(&mut buf);
        match result {
            Ok(n) => {
                println!("{} bytes read from interface", n);

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
                }
            }
            Err(err) => {
                println!("{} error when reading from interface", err)
            }
        }
        //sleep(Duration::from_micros(20));
    }
}
