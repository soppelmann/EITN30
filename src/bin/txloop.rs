use std::thread::sleep;
use std::time::Duration;
use std::io::{Read, Write};
use std::thread;
use eitn_30 as lib;

use nrf24l01::{OperatingMode, PALevel, TXConfig, NRF24L01};

fn main() {
    let mut device = lib::tx_setup(108, *b"abcde", 17, 0, 0);

    let message = b"sendtest";
    loop {
        device.push(0, message).unwrap();
        match device.send() {
            Ok(retries) => {
                println!("Message sent, {} retries needed", retries);
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
                println!("Destination unreachable: {:?}", err);
                device.flush_output().unwrap()
            }
        };
        sleep(Duration::from_millis(5000));
    }
}
