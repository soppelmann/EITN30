use crate::rx_setup;
use crate::BUFFER_SIZE;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn rx_loop(mut writer: Writer) {
    let mut device = rx_setup(108, *b"abcde", 27, 1, 0);
    let mut buf = [0u8; BUFFER_SIZE];
    let mut end;
    let mut skip;
    loop {
        end = 0;
        skip = true;
        while skip || packet::ip::Packet::new(&buf[..end]).is_err() {
            sleep(Duration::from_millis(1));
            match device.data_available() {
                Ok(true) => {
                    skip = false;
                    device
                        .read_all(|packet| {
                            println!("Received {:?} bytes", packet.len());
                            println!("Payload {}", String::from_utf8_lossy(packet));
                            let start = end;
                            end += packet.len();
                            buf[start..end].copy_from_slice(packet);
                        })
                        .unwrap();
                }

                Ok(false) => {
                    //println!("No data available");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        println!("Writing {} bytes to interface", end);
        writer.write(&buf[..end]).unwrap();
    }
}
