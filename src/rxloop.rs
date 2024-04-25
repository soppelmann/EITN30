use crate::{BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE};
use nrf24l01::NRF24L01;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn rx_loop(mut device: NRF24L01, mut writer: Writer) {
    let mut buf = [0u8; BUFFER_SIZE];
    let mut total_length: u16 = 3;
    let mut end;
    let mut init;
    loop {
        end = 0;
        init = true;
        loop {
            sleep(Duration::from_micros(10));
            // Avoid buffer overflow in case of failure.
            if end + PACKET_SIZE * QUEUE_SIZE >= BUFFER_SIZE {
                println!("Something terrible happened!");
                end = 0;
            }

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

            // Better ipv4 is_err(check)
            // Need to treat bytes 2 and 3 as one long number
            // see: https://en.wikipedia.org/wiki/Internet_Protocol_version_4#Total_Length
            if init && end == 32 {
                let length_slice = &buf[2..4];
                total_length = u16::from_be_bytes([length_slice[0], length_slice[1]]);
                init = false;
                //println!("This packet should be size: {:?}", total_length);
            }

            if !init && end == total_length as usize {
                //println!("Writing {} bytes to interface", end);
                // Probably want to make this async
                _ = writer.write(&buf[..end]).unwrap();
                break;
            }
        }
    }
}
