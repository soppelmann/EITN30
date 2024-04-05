use std::thread::sleep;
use std::time::Duration;

use nrf24l01::{OperatingMode, PALevel, RXConfig, NRF24L01};

fn rx_setup(chan: u8, address: [u8; 5], pin: u64, port: u8, device: u8) -> NRF24L01 {
    if address.len() != 5 {
        panic!("Pipe0 address should be 5 bytes long");
    }

    let config = RXConfig {
        channel: chan,
        pa_level: PALevel::Min,
        pipe0_address: address,
        ..Default::default()
    };

    let mut device = NRF24L01::new(pin, port, device).unwrap();
    device.configure(&OperatingMode::RX(config)).unwrap();
    device.listen().unwrap();
    device.flush_output().unwrap();
    device.flush_input().unwrap();

    return device;
}

fn main() {
    let mut device = rx_setup(108, *b"abcde", 17, 0, 0);

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
