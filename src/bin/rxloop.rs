use std::thread::sleep;
use std::time::Duration;

use nrf24l01::{OperatingMode, PALevel, RXConfig, NRF24L01};

fn main() {
    //let config_1 = RXConfig {
    //    channel: 108,
    //    pa_level: PALevel::Min,
    //    pipe0_address: *b"abcde",
    //    ..Default::default()
    //};

    let config = RXConfig {
        channel: 108,
        pa_level: PALevel::Min,
        pipe0_address: *b"abcde",
        ..Default::default()
    };

    //let mut device_1 = NRF24L01::new(17, 0, 0).unwrap();
    let mut device = NRF24L01::new(17, 0, 0).unwrap();

    device.configure(&OperatingMode::RX(config)).unwrap();
    device.listen().unwrap();
    loop {
        sleep(Duration::from_millis(2000));
        if device.data_available().unwrap() {
            device
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    println!("Payload {:?}", packet);
                    println!("Payload: {:?}", String::from_utf8_lossy(packet));
                })
                .unwrap();
            if let Err(err) = device.push(0, b"ack payload") {
                println!("Error while pushing payload: {:?}", err);
            } else {
                // prepare ack payload for next reception
                device.push(0, b"ack payload").unwrap();
            }
        }
    }
}
