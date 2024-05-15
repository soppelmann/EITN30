use crate::{BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE, TX_RETRIES};
use nrf24l01::NRF24L01;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Reader;

pub fn transmit(mut device: NRF24L01) -> NRF24L01 {
    for i in 0..TX_RETRIES {
        match device.send() {
            Ok(retries) => {
                println!("message sent, {} retries needed", retries);
                break;
            }
            Err(err) => {
                println!("destination unreachable: {:?}", err);
                if i == 2 {
                    println!("max retries reached: {}, flushing output", i);
                    device.flush_output().unwrap();
                }
                println!("reached: {}", i);
            }
        };
    }
    device
}

pub fn tx_loop(mut device: NRF24L01, mut reader: Reader) {
    loop {
        let mut buf = [0u8; BUFFER_SIZE];
        let result = reader.read(&mut buf);
        match result {
            Ok(n) => {
                let pkt = &buf[0..n];
                // Split the packet into 32 byte chunks that can we push to the device
                for queue in pkt.chunks(PACKET_SIZE * QUEUE_SIZE) {
                    queue.chunks(PACKET_SIZE).for_each(|pkt| {
                        // push the packet to the 3 length FIFO_QUEUE
                        device.push(0, pkt).unwrap();
                    });
                    // attempt transmit using device.send()
                    // We need to return the object as we cant move after borrow in the loop
                    // Basically, the transmit function has to return the device to the tx_loop function
                    // when its doing playing with it.
                    device = transmit(device);
                }
            }
            Err(err) => {
                println!("{} error when reading from interface", err)
            }
        }
        sleep(Duration::from_micros(1));
    }
}
