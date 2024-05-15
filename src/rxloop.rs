use crate::{BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE};
use nrf24l01::NRF24L01;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn rx_loop(mut device: NRF24L01, writer: Arc<Mutex<Writer>>) {
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

            // Fill a buffer with received data
            match device.data_available() {
                Ok(true) => {
                    device
                        .read_all(|packet| {
                            let start = end;
                            end += packet.len();
                            buf[start..end].copy_from_slice(packet);
                        })
                        .unwrap();
                }

                Ok(false) => {}
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

            // Better ipv4 is_err(check)
            // We find out the Total Length of the packet using bytes 2 and 3.
            // see: https://en.wikipedia.org/wiki/Internet_Protocol_version_4#Total_Length
            if init && end <= 32 {
                let length_slice = &buf[2..4];
                total_length = u16::from_be_bytes([length_slice[0], length_slice[1]]);
                init = false;
            }

            // Write if its not first iteration and we have received the amount
            // of bytes that was listed in the IPv4 Total leangth field in the
            // header. We need to create a mutex and massage rust but it works
            if !init && end == total_length as usize {
                let tun_writer_clone = writer.clone();
                let _tun_writer = thread::spawn(move || {
                    let mut writer_ref = tun_writer_clone.lock().unwrap(); // Get a mutable reference
                    _ = writer_ref.write(&buf[..end]).unwrap();
                });

                break;
            }
        }
    }
}
