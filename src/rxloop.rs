use crate::rx_setup;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Reader;

pub fn rx_loop(mut reader: Reader) {
    let mut device = rx_setup(108, *b"abcde", 27, 1, 0);
    let mut buf = [0u8; 4096];
    loop {
        sleep(Duration::from_millis(1));
        if device.data_available().unwrap() {
            device
                .read_all(|packet| {
                    println!("Received {:?} bytes", packet.len());
                    //println!("Payload {:?}", packet);
                    println!("Payload {}", String::from_utf8_lossy(packet));
                    //reader.read_exact(&mut buf).unwrap();
                })
                .unwrap();
            let result = reader.read(&mut buf);
            match result {
                Ok(n) => println!("{} bytes read from interface", n),
                Err(err) => {
                    println!("{} error when reading from interface", err)
                }
            }
        }
    }
}
