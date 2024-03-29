//use std::io;

use embedded_hal::delay::DelayNs;
use embedded_nrf24l01::{Configuration, CrcMode, DataRate, NRF24L01, PIPES_COUNT, Payload, RxMode, Device};

use linux_embedded_hal::spidev::{self, SpidevOptions};
//use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay as delay;
//use linux_embedded_hal::{SysfsPin, SpidevDevice};
use linux_embedded_hal::{CdevPin, SpidevDevice};

use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};

// use ufmt::{derive::uDebug, uwrite};

fn main() {
    // Configure SPI https://docs.rs/ssd1675/latest/ssd1675/interface/struct.Interface.html
    let mut spi = SpidevDevice::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // Configure CE pin
    let mut chip = Chip::new("/dev/gpiochip4").unwrap();
    let handle = chip.get_line(27).unwrap(); // Get LineHandle using pin number
    let req_flags = LineRequestFlags::OUTPUT;
    let ce = match CdevPin::new(handle.request(req_flags, 0, "my_ce_pin").unwrap()) {
        Ok(pin) => pin,
        Err(error) => panic!("Failed to create CdevPin: {}", error), // Handle the error
    };

    println!("Hello, world!");

    // this will be useful later https://github.com/astro/embedded-nrf24l01/pull/11/files
    let mut nrf24 = NRF24L01::new(ce, spi).unwrap();

    // https://github.com/astro/embedded-nrf24l01/issues/12
    //let mut nrf24 = nrf24.tx().unwrap(); //default configuration from example

    // nrf24.set_frequency(8).unwrap();
    // nrf24.set_auto_retransmit(15, 15).unwrap();
    // nrf24.set_rf(&DataRate::R2Mbps, 0).unwrap();
    // nrf24
    //     .set_pipes_rx_enable(&[true, false, false, false, false, false])
    //     .unwrap();
    // nrf24
    //     .set_auto_ack(&[true, false, false, false, false, false])
    //     .unwrap();
    // nrf24.set_pipes_rx_lengths(&[None; 6]).unwrap();
    // nrf24.set_crc(CrcMode::TwoBytes).unwrap();
    // nrf24.set_rx_addr(0, &b"fnord"[..]).unwrap();
    // nrf24.set_tx_addr(&b"fnord"[..]).unwrap();
    // nrf24.flush_rx().unwrap();
    // nrf24.flush_tx().unwrap();

    // https://github.com/burgerindividual/attiny-neopixel-receiver/blob/main/src/main.rs#L111
    nrf24.set_rf(&DataRate::R250Kbps, 2).unwrap();
    nrf24.set_frequency(13).unwrap();
    nrf24.set_address_width(3).unwrap();
    nrf24.set_crc(CrcMode::OneByte).unwrap();
    nrf24.set_rx_addr(0, b"nrf").unwrap();
    nrf24.set_auto_ack(&[false; PIPES_COUNT]).unwrap();
    nrf24.set_pipes_rx_enable(&[true, false, false, false, false, false])
        .unwrap();
    nrf24.set_pipes_rx_lengths(&[None; PIPES_COUNT]).unwrap();
    nrf24.set_interrupt_mask(false, true, true).unwrap();
    nrf24.clear_interrupts().unwrap();

    // Print addresswidth
    println!("Address width is: {}", nrf24.get_address_width().unwrap());

    // Receive
    let mut nrf24 = nrf24.rx().unwrap(); //default configuration from example
    let mut buf: [u8; 4] = [0; 4];
    delay.delay_us(130u32);

    loop {
       println!("Receiving data...");
       if nrf24.can_read().is_ok() {
           let payload = nrf24.read();
           match payload {
               Ok(p) => {
                   //println!("Got payload = {}", p.as_ref());
                   buf.copy_from_slice(p.as_ref());
                   let num = u32::from_le_bytes(buf);
                   println!("Got message = {:?}", num);
               }
               Err(_) => {
                   println!("Could not read payload");
               }
           }
       }
    }

    // loop {
    //     delay.delay_us(500u32);
    //     if nrf24.can_read().is_ok() {
    //         let payload = nrf24.read();
    //         match payload {
    //             Ok(p) => {
    //                 println!("Got payload = {:?}", p.as_ref());
    //             }
    //             Err(_) => {
    //                 println!("Could not read payload");
    //             }
    //         }
    //     }
    // }

    // loop {
    //     // process if RX ready interrupt is set
    //     if nrf24.get_interrupts().unwrap().0 {
    //         let payload = nrf24.read().unwrap();
    //         nrf24.clear_interrupts().unwrap();

    //         let decoded = core::str::from_utf8(&payload).unwrap();

    //         // print the received data
    //         println!("Received: {}", decoded);

    //     }
    //     // waiting for atleast 250μs ends the led line commands. we'll wait for 300μs
    //     delay.delay_us(500u32);
    //     //delay_cycles(4800);
    //     // delay for 2ms, this is the maximum refresh speed of the led strip
    //     // (this can be shorter)
    //     // delay_cycles(16000 * 2);
    // }

}
