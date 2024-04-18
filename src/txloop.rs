use crate::tx_setup;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn tx_loop(mut writer: Writer) {
    let mut device = tx_setup(108, *b"abcde", 17, 0, 0);

    let message = b"sendtest";
    loop {
        device.push(0, message).unwrap();
        match device.send() {
            Ok(retries) => {
                println!("Message sent, {} retries needed", retries);
                writer.write(message).unwrap();
                if device.data_available().unwrap() {
                    device
                        .read_all(|packet| {
                            println!("Received back {:?} bytes", packet.len());
                            println!("ACK Payload {}", String::from_utf8_lossy(packet));
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
