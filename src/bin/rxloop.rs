use std::thread::sleep;
use std::time::Duration;
use std::io::{Read, Write};
use std::thread;
use eitn_30 as lib;

use nrf24l01::{OperatingMode, PALevel, RXConfig, NRF24L01};

fn main() {
    let mut device = lib::rx_setup(108, *b"abcde", 17, 0, 0);

    loop {
        sleep(Duration::from_millis(500));
        if device.data_available().unwrap() {
            device
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    println!("Payload {:?}", packet);
                })
                .unwrap();
            // prepare ack payload for next reception
            device.push(0, b"ack payload").unwrap();
        }
    }
}
