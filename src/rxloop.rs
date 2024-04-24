use crate::BUFFER_SIZE;
use nrf24l01::NRF24L01;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn rx_loop(mut device: NRF24L01, mut writer: Writer) {
    let mut buf = [0u8; BUFFER_SIZE];
    let mut end;
    loop {
        end = 0;
        // the .is_err() is a very slow function see flamegraph
        while end <= 20 || packet::ip::v4::Packet::new(&buf[..end]).is_err() {
            sleep(Duration::from_micros(10));
            if end + 96 >= BUFFER_SIZE {
                println!("Something terrible happened!");
                end = 0;
            }
            //sleep(Duration::from_micros(10));
            match device.data_available() {
                Ok(true) => {
                    device
                        .read_all(|packet| {
                            // println!("Received {:?} bytes", packet.len());
                            // println!("Payload {}", String::from_utf8_lossy(packet));
                            let start = end;
                            end += packet.len();
                            buf[start..end].copy_from_slice(packet);
                        })
                        .unwrap();
                }

                Ok(false) => {
                    //println!("No data available"); //this causes spam
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        println!("Writing {} bytes to interface", end);
        _ = writer.write(&buf[..end]).unwrap();
    }
}
