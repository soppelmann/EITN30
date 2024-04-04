use std::thread::sleep;
use std::time::Duration;

use nrf24l01::{OperatingMode, PALevel, TXConfig, NRF24L01};

fn main() {
    let config = TXConfig {
        channel: 108,
        pa_level: PALevel::Min,
        pipe0_address: *b"abcde",
        max_retries: 255,
        retry_delay: 2,
        ..Default::default()
    };
    let mut device = NRF24L01::new(17, 0, 0).unwrap();
    let message = b"sendtest";
    device.configure(&OperatingMode::TX(config)).unwrap();
    device.flush_output().unwrap();
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
                            println!("ACK Payload: {}", String::from_utf8(packet));
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
