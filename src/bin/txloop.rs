use std::thread::sleep;
use std::time::Duration;

use nrf24l01::{OperatingMode, PALevel, TXConfig, NRF24L01};

// Function that configures TXConfig and takes channel, pipe0_address, ce_pin, spi_port, spi_device as arguemnt
fn tx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = TXConfig {
        channel: chan,
        pa_level: PALevel::Min,
        pipe0_address: address,
        max_retries: 255,
        retry_delay: 2,
        ..Default::default()
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::TX(config)).unwrap();
    device.flush_output().unwrap();
    device.flush_input().unwrap();

    return device;
}

fn main() {
    let mut device = tx_setup(108, *b"abcde", 17, 0, 0);

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
