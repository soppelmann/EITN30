use std::thread::sleep;
use std::time::Duration;

use nrf24l01::{OperatingMode, PALevel, RXConfig, NRF24L01};

fn main() {
    let config_1 = RXConfig {
        channel: 108,
        pa_level: PALevel::Min,
        pipe0_address: *b"abcde",
        ..Default::default()
    };

    let config_2 = RXConfig {
        channel: 108,
        pa_level: PALevel::Min,
        pipe0_address: *b"abcde",
        ..Default::default()
    };

    let mut device_1 = NRF24L01::new(17, 0, 0).unwrap();
    let mut device_2 = NRF24L01::new(27, 1, 1).unwrap();
    

    device_2.configure(&OperatingMode::RX(config_2)).unwrap();
    device_2.listen().unwrap();
    loop {
        sleep(Duration::from_millis(500));
        if device_2.data_available().unwrap() {
            device_2
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    println!("Payload {:?}", packet);
                    println!("Payload: {:?}", String::from_utf8_lossy(packet));
                })
                .unwrap();
            // prepare ack payload for next reception
            device_2.push(0, b"ack payload").unwrap();
        }
    }
}
