use crate::rx_setup;
use std::thread::sleep;
use std::time::Duration;

pub fn rx_loop() {
    let mut device = rx_setup(108, *b"abcde", 27, 1, 0);

    loop {
        sleep(Duration::from_millis(500));
        if device.data_available().unwrap() {
            device
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    println!("Payload {}", String::from_utf8_lossy(packet));
                })
                .unwrap();
            // prepare ack payload for next reception
            device.push(0, b"ack payload").unwrap();
        }
    }
}
