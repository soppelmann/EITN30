use crate::{ BUFFER_SIZE, PACKET_SIZE, QUEUE_SIZE };
use nrf24l01::NRF24L01;
use std::io::Write;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tun2::platform::posix::Writer;

pub fn rx_loop(mut device: NRF24L01, writer: Arc<Mutex<Writer>>) {
    let mut buf = [0u8; BUFFER_SIZE];
    let mut end;
    //let mut init;
    loop {
        end = 0;
        //init = true;
        while end <= 20 || packet::ip::v4::Packet::new(&buf[..end]).is_err() {
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
        }

        // Write if its not first iteration and we have received the amount
        // of bytes that was listed in the IPv4 Total leangth field in the
        // header. We need to create a mutex and massage rust but it works
        let tun_writer_clone = writer.clone();
        let _tun_writer = thread::spawn(move || {
            let mut writer_ref = tun_writer_clone.lock().unwrap(); // Get a mutable reference
            _ = writer_ref.write(&buf[..end]).unwrap();
        });
    }
}
